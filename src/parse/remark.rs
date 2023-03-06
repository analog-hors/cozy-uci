use std::time::Duration;

use crate::remark::*;
use crate::UciFormatOptions;

use super::error::{UciParseError, UciParseErrorKind};
use super::stream::UciTokenStream;
use UciParseErrorKind::*;

impl UciRemark {
    pub fn parse_from(s: &str, options: &UciFormatOptions) -> Result<Self, UciParseError> {
        let mut s = UciTokenStream::new(s);
        let (rmk, rmk_span) = s.read_token()?;
        let rmk = match rmk {
            "id" => match s.read_token()? {
                ("name", _) => Self::Id(UciIdInfo::Name(s.read_string(|tok| tok.is_none())?)),
                ("author", _) => Self::Id(UciIdInfo::Author(s.read_string(|tok| tok.is_none())?)),
                (tok, span) => Err(UnexpectedToken(tok.to_owned()).spans(span))?,
            },
            "uciok" => Self::UciOk,
            "readyok" => Self::ReadyOk,
            "bestmove" => {
                let mv = s.read_type()?;
                let ponder = match s.read_token() {
                    Ok(("ponder", _)) => Some(s.read_type()?),
                    Ok((tok, span)) => Err(UnexpectedToken(tok.to_owned()).spans(span))?,
                    Err(_) => None,
                };
                Self::BestMove { mv, ponder }
            }
            "info" => Self::Info(read_info(&mut s, options)?),
            "option" => {
                s.expect_token("name")?;
                let name = s.read_string(|tok| tok == Some("type"))?;
                let info = read_option_info(&mut s)?;
                Self::Option { name, info }
            }
            rmk => Err(UnknownMessageKind(rmk.to_owned()).spans(rmk_span))?,
        };
        s.expect_end()?;
        Ok(rmk)
    }
}

fn read_info(s: &mut UciTokenStream, options: &UciFormatOptions) -> Result<UciInfo, UciParseError> {
    macro_rules! parse_info {
        ($($field:ident => $body:expr,)*) => {{
            use UciParseErrorKind::*;

            #[allow(non_upper_case_globals, unused)]
            mod ident_to_str {
                $(pub const $field: &str = stringify!($field);)*
            }

            let mut info = UciInfo::default();
            while let Ok((field, span)) = s.read_token() {
                match field {
                    $(ident_to_str::$field => {
                        if info.$field.is_some() {
                            Err(DuplicateField(ident_to_str::$field).spans(span))?;
                        }
                        info.$field = Some($body);
                    })*
                    _ => Err(UnknownField(field.to_owned()).spans(span))?
                }
            }
            Ok(info)
        }};
    }

    parse_info! {
        depth => s.read_type()?,
        seldepth => s.read_type()?,
        time => Duration::from_millis(s.read_type()?),
        nodes => s.read_type()?,
        pv => s.read_moves(),
        multipv => s.read_type()?,
        score => read_uci_score(s, options)?,
        currmove => s.read_type()?,
        currmovenumber => s.read_type()?,
        hashfull => s.read_type()?,
        nps => s.read_type()?,
        tbhits => s.read_type()?,
        sbhits => s.read_type()?,
        cpuload => s.read_type()?,
        string => s.read_string(|s| s.is_none())?,
        refutation => s.read_moves(),
        currline => {
            let mut cpu = None;
            if let Ok(Ok(num)) = s.peek_token().map(|(tok, _)| tok.parse()) {
                let _ = s.read_token();
                cpu = Some(num);
            }
            let moves = s.read_moves();
            UciCurrline { cpu, moves }
        },
    }
}

fn read_option_info(s: &mut UciTokenStream) -> Result<UciOptionInfo, UciParseError> {
    s.expect_token("type")?;
    let (tok, span) = s.read_token()?;
    Ok(match tok {
        "check" => {
            s.expect_token("default")?;
            let default = s.read_bool("true", "false")?;
            UciOptionInfo::Check { default }
        }
        "spin" => {
            s.expect_token("default")?;
            let default = s.read_type()?;
            s.expect_token("min")?;
            let min = s.read_type()?;
            s.expect_token("max")?;
            let max = s.read_type()?;
            UciOptionInfo::Spin { default, min, max }
        }
        "combo" => {
            s.expect_token("default")?;
            let default = s.read_token()?.0.to_owned();
            let mut labels = Vec::new();
            while s.peek_token().is_ok() {
                s.expect_token("var")?;
                labels.push(s.read_token()?.0.to_owned());
            }
            UciOptionInfo::Combo { default, labels }
        }
        "button" => UciOptionInfo::Button,
        "string" => {
            s.expect_token("default")?;
            let default = s.read_string(|tok| tok.is_none())?;
            UciOptionInfo::String { default }
        }
        tok => Err(UnexpectedToken(tok.to_owned()).spans(span))?,
    })
}

fn read_uci_score(
    s: &mut UciTokenStream,
    options: &UciFormatOptions,
) -> Result<UciScore, UciParseError> {
    let mut cp = None;
    let mut mate = None;
    let mut wdl = None;
    let mut kind = None;
    while let Ok((tok, span)) = s.peek_token() {
        let duplicate = match tok {
            "cp" => {
                let _ = s.read_token();
                cp.replace(s.read_type()?).is_some()
            }
            "mate" => {
                let _ = s.read_token();
                mate.replace(s.read_type()?).is_some()
            }
            "wdl" if options.wdl => {
                let _ = s.read_token();
                let w = s.read_type()?;
                let d = s.read_type()?;
                let l = s.read_type()?;
                wdl.replace((w, d, l)).is_some()
            }
            "lowerbound" => {
                let _ = s.read_token();
                kind.replace(UciScoreKind::LowerBound).is_some()
            }
            "upperbound" => {
                let _ = s.read_token();
                kind.replace(UciScoreKind::UpperBound).is_some()
            }
            _ => break,
        };
        if duplicate {
            return Err(InvalidField("score").spans(span));
        }
    }
    let kind = kind.unwrap_or(UciScoreKind::Exact);
    Ok(UciScore {
        cp,
        mate,
        wdl,
        kind,
    })
}

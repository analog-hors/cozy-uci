use std::time::Duration;

use crate::command::*;
use crate::UciFormatOptions;

use super::error::{UciParseError, UciParseErrorKind};
use super::stream::UciTokenStream;

impl UciCommand {
    pub fn parse_from(s: &str, options: &UciFormatOptions) -> Result<Self, UciParseError> {
        use UciParseErrorKind::*;

        let mut s = UciTokenStream::new(s);
        let (cmd, cmd_span) = s.read_token()?;
        let cmd = match cmd {
            "uci" => Self::Uci,
            "debug" => Self::Debug(s.read_bool("on", "off")?),
            "isready" => Self::IsReady,
            "position" => {
                let (tok, span) = s.read_token()?;
                let init_pos = match tok {
                    "fen" => UciInitPos::Board(s.read_fen(options.chess960)?),
                    "startpos" => UciInitPos::StartPos,
                    tok => Err(UnexpectedToken(tok.to_owned()).spans(span))?,
                };
                let mut moves = Vec::new();
                if s.peek_token().is_ok() {
                    s.expect_token("moves")?;
                    moves = s.read_moves();
                }
                Self::Position { init_pos, moves }
            }
            "setoption" => {
                s.expect_token("name")?;
                let name = s.read_string(|tok| matches!(tok, Some("value") | None))?;
                let mut value = None;
                if s.peek_token().is_ok() {
                    s.expect_token("value")?;
                    value = Some(s.read_string(|tok| tok.is_none())?);
                }
                Self::SetOption { name, value }
            }
            "ucinewgame" => Self::UciNewGame,
            "stop" => Self::Stop,
            "ponderhit" => Self::PonderHit,
            "quit" => Self::Quit,
            "go" => Self::Go(read_go_params(&mut s)?),
            cmd => Err(UnknownMessageKind(cmd.to_owned()).spans(cmd_span))?,
        };
        s.expect_end()?;
        Ok(cmd)
    }
}

fn read_go_params(s: &mut UciTokenStream) -> Result<UciGoParams, UciParseError> {
    use UciParseErrorKind::*;

    let mut params = UciGoParams::default();

    macro_rules! parse_go_params {
        (
            $field_ident:ident, $span_ident:ident;
            $([$($processed_field:ident => $processed_body:expr,)*],)?
            $field:ident => $body:expr,
            $($tail:tt)*
        ) => {
            parse_go_params! {
                $field_ident, $span_ident;
                [
                    $($($processed_field => $processed_body,)*)*
                    $field => $body,
                ],
                $($tail)*
            }
        };

        (
            $field_ident:ident, $span_ident:ident;
            $([$($processed_field:ident => $processed_body:expr,)*],)?
            $field:ident -> $body:expr,
            $($tail:tt)*
        ) => {
            parse_go_params! {
                $field_ident, $span_ident;
                [
                    $($($processed_field => $processed_body,)*)*
                    $field => {
                        if params.$field.is_some() {
                            Err(DuplicateField(ident_to_str::$field).spans($span_ident))?;
                        }
                        params.$field = Some($body);
                    },
                ],
                $($tail)*
            }
        };

        (
            $field_ident:ident, $span_ident:ident;
            [$($field:ident => $body:expr,)*],
        ) => {
            #[allow(non_upper_case_globals, unused)]
            mod ident_to_str {
                $(pub const $field: &str = stringify!($field);)*
            }

            while let Ok(($field_ident, $span_ident)) = s.read_token() {
                match $field_ident {
                    $(ident_to_str::$field => $body)*
                    _ => Err(UnknownField($field_ident.to_owned()).spans($span_ident))?
                }
            }
        };
    }

    parse_go_params! {
        field, span;
        searchmoves -> s.read_moves(),
        ponder => {
            if params.ponder {
                Err(DuplicateField("ponder").spans(span))?;
            }
            params.ponder = true;
        },
        wtime -> Duration::from_millis(s.read_type()?),
        btime -> Duration::from_millis(s.read_type()?),
        winc -> Duration::from_millis(s.read_type()?),
        binc -> Duration::from_millis(s.read_type()?),
        movestogo -> s.read_type()?,
        depth -> s.read_type()?,
        nodes -> s.read_type()?,
        mate -> s.read_type()?,
        movetime -> Duration::from_millis(s.read_type()?),
        infinite => {
            if params.infinite {
                Err(DuplicateField("infinite").spans(span))?;
            }
            params.infinite = true;
        },
    }
    Ok(params)
}

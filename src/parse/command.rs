use std::time::Duration;

use crate::command::*;
use crate::UciFormatOptions;

use super::stream::{UciParseError, UciTokenStream};

impl UciCommand {
    pub fn parse_from(s: &str, options: &UciFormatOptions) -> Result<Self, UciParseError> {
        use UciParseError::*;

        let mut s = UciTokenStream::new(s);
        let cmd = match s.read_token()? {
            "uci" => Self::Uci,
            "debug" => match s.read_token()? {
                "on" => Self::Debug(true),
                "off" => Self::Debug(false),
                tok => Err(UnexpectedToken(tok.to_owned()))?,
            },
            "isready" => Self::IsReady,
            "position" => {
                let init_pos = match s.read_token()? {
                    "fen" => UciInitPos::Board(s.read_fen(options.chess960)?),
                    "startpos" => UciInitPos::StartPos,
                    tok => Err(UnexpectedToken(tok.to_owned()))?,
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
            cmd => Err(UnknownMessageKind(cmd.to_owned()))?,
        };
        s.expect_end()?;
        Ok(cmd)
    }
}

fn read_go_params(s: &mut UciTokenStream) -> Result<UciGoParams, UciParseError> {
    use UciParseError::*;

    let mut params = UciGoParams::default();

    macro_rules! parse_go_params {
        (
            $([$($processed_field:ident => $processed_body:expr,)*],)?
            $field:ident => $body:expr,
            $($tail:tt)*
        ) => {
            parse_go_params! {
                [
                    $($($processed_field => $processed_body,)*)*
                    $field => $body,
                ],
                $($tail)*
            }
        };

        (
            $([$($processed_field:ident => $processed_body:expr,)*],)?
            $field:ident -> $body:expr,
            $($tail:tt)*
        ) => {
            parse_go_params! {
                [
                    $($($processed_field => $processed_body,)*)*
                    $field => {
                        if params.$field.is_some() {
                            Err(DuplicateField(ident_to_str::$field))?;
                        }
                        params.$field = Some($body);
                    },
                ],
                $($tail)*
            }
        };

        ([$($field:ident => $body:expr,)*],) => {
            #[allow(non_upper_case_globals, unused)]
            mod ident_to_str {
                $(pub const $field: &str = stringify!($field);)*
            }

            while let Ok(field) = s.read_token() {
                match field {
                    $(ident_to_str::$field => $body)*
                    _ => Err(UnknownField(field.to_owned()))?
                }
            }
        };
    }

    parse_go_params! {
        searchmoves -> s.read_moves(),
        ponder => {
            if params.ponder {
                Err(DuplicateField("ponder"))?;
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
                Err(DuplicateField("infinite"))?;
            }
            params.infinite = true;
        },
    }
    Ok(params)
}

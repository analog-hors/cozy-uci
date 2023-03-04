use crate::UciFormatOptions;
use crate::command::*;

use std::fmt::{Display, Formatter};

struct UciCommandFormatter<'f> {
    command: &'f UciCommand,
    options: &'f UciFormatOptions
}

impl UciCommand {
    pub fn format<'f>(&'f self, options: &'f UciFormatOptions) -> String {
        format!("{}", UciCommandFormatter { command: self, options })
    }
}

impl Display for UciCommandFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use UciCommand::*;

        match self.command {
            Uci => write!(f, "uci")?,
            Debug(on) => write!(f, "debug {}", if *on { "on" } else { "off" })?,
            IsReady => write!(f, "isready")?,
            Position { init_pos, moves } => {
                write!(f, "position ")?;
                match init_pos {
                    UciInitPos::StartPos => write!(f, "startpos")?,
                    UciInitPos::Board(board) => {
                        match self.options.chess960 {
                            false => write!(f, "fen {}", board)?,
                            true => write!(f, "fen {:#}", board)?
                        }
                    }
                }
                if !moves.is_empty() {
                    write!(f, " moves")?;
                    for mv in moves {
                        write!(f, " {}", mv)?;
                    }
                }
            },
            SetOption { name, value } => {
                write!(f, "setoption name {}", name)?;
                if let Some(value) = value {
                    write!(f, " value {}", value)?;
                }
            },
            UciNewGame => write!(f, "ucinewgame")?,
            Stop => write!(f, "stop")?,
            PonderHit => write!(f, "ponderhit")?,
            Quit => write!(f, "quit")?,
            Go(params) => format_go_params(f, params)?,
        }
        Ok(())
    }
}

fn format_go_params(f: &mut Formatter, params: &UciGoParams) -> std::fmt::Result {
    macro_rules! format_go_params {
        ($field:ident -> $body:expr, $($tail:tt)*) => {
            if let Some($field) = &params.$field {
                $body
            }
            format_go_params!($($tail)*);
        };

        ($field:ident => $body:expr, $($tail:tt)*) => {
            {
                let $field = &params.$field;
                $body
            }
            format_go_params!($($tail)*);
        };

        ($field:ident, $($tail:tt)*) => {
            format_go_params! {
                $field -> write!(f, " {} {}", stringify!($field), $field)?,
                $($tail)*
            }
        };

        () => {};
    }
    
    write!(f, "go")?;
    format_go_params! {
        searchmoves -> {
            write!(f, " searchmoves")?;
            for mv in searchmoves {
                write!(f, " {}", mv)?;
            }
        },
        ponder => if *ponder { write!(f, " ponder")? },
        wtime -> write!(f, " wtime {}", wtime.as_millis())?,
        btime -> write!(f, " btime {}", btime.as_millis())?,
        winc -> write!(f, " winc {}", winc.as_millis())?,
        binc -> write!(f, " binc {}", binc.as_millis())?,
        movestogo,
        depth,
        nodes,
        mate,
        movetime -> write!(f, " movetime {}", movetime.as_millis())?,
        infinite => if *infinite { write!(f, " infinite")? },
    }
    Ok(())
}

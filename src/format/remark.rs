use crate::remark::*;
use crate::UciFormatOptions;

use std::fmt::{Display, Formatter};

struct UciRemarkFormatter<'f> {
    remark: &'f UciRemark,
    options: &'f UciFormatOptions,
}

impl UciRemark {
    pub fn format<'f>(&'f self, options: &'f UciFormatOptions) -> String {
        format!(
            "{}",
            UciRemarkFormatter {
                remark: self,
                options
            }
        )
    }
}

impl Display for UciRemarkFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use UciIdInfo::*;
        use UciOptionInfo::*;
        use UciRemark::*;

        match self.remark {
            Id(Name(name)) => write!(f, "id name {}", name)?,
            Id(Author(author)) => write!(f, "id author {}", author)?,
            UciOk => write!(f, "uciok")?,
            ReadyOk => write!(f, "readyok")?,
            BestMove { mv, ponder } => {
                write!(f, "bestmove {}", mv)?;
                if let Some(mv) = ponder {
                    write!(f, " ponder {}", mv)?;
                }
            }
            Info(info) => self.format_info(f, info)?,
            Option { name, info } => {
                write!(f, "option name {} type ", name)?;
                match info {
                    Check { default } => write!(f, "check default {}", default)?,
                    Spin { default, min, max } => {
                        write!(f, "spin default {} min {} max {}", default, min, max)?
                    }
                    Combo { default, labels } => {
                        write!(f, "combo default {}", default)?;
                        for label in labels {
                            write!(f, " var {}", label)?;
                        }
                    }
                    Button => write!(f, "button")?,
                    String { default } => write!(f, "string default {}", default)?,
                }
            }
        }
        Ok(())
    }
}

impl UciRemarkFormatter<'_> {
    fn format_info(&self, f: &mut Formatter, info: &UciInfo) -> std::fmt::Result {
        macro_rules! format_info_fields {
            ($field:ident -> $body:expr, $($tail:tt)*) => {
                if let Some($field) = &info.$field {
                    $body
                }
                format_info_fields!($($tail)*);
            };

            ($field:ident, $($tail:tt)*) => {
                format_info_fields! {
                    $field -> write!(f, " {} {}", stringify!($field), $field)?,
                    $($tail)*
                }
            };

            () => {};
        }

        write!(f, "info")?;
        format_info_fields! {
            depth,
            seldepth,
            time -> write!(f, " time {}", time.as_millis())?,
            nodes,
            pv -> {
                write!(f, " pv")?;
                for mv in pv {
                    write!(f, " {}", mv)?;
                }
            },
            multipv,
            score -> {
                write!(f, " score")?;
                if let Some(cp) = score.cp {
                    write!(f, " cp {}", cp)?;
                }
                if let Some(mate) = score.mate {
                    write!(f, " mate {}", mate)?;
                }
                if let Some((w, d, l)) = score.wdl {
                    if self.options.wdl {
                        write!(f, " wdl {} {} {}", w, d, l)?;
                    }
                }
                match score.kind {
                    UciScoreKind::LowerBound => write!(f, " lowerbound")?,
                    UciScoreKind::UpperBound => write!(f, " upperbound")?,
                    _ => {}
                }
            },
            currmove,
            currmovenumber,
            hashfull,
            nps,
            tbhits,
            sbhits,
            cpuload,
            string,
            refutation -> {
                write!(f, " refutation")?;
                for mv in refutation {
                    write!(f, " {}", mv)?;
                }
            },
            currline -> {
                write!(f, " currline")?;
                if let Some(cpu) = currline.cpu {
                    write!(f, " {}", cpu)?;
                }
                for mv in &currline.moves {
                    write!(f, " {}", mv)?;
                }
            },
        }
        Ok(())
    }
}

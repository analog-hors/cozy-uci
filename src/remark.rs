use std::time::Duration;

use cozy_chess::Move;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UciIdInfo {
    Name(String),
    Author(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UciScore {
    pub cp: Option<i32>,
    pub mate: Option<i32>,
    pub wdl: Option<(u16, u16, u16)>,
    pub kind: UciScoreKind
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UciScoreKind {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UciCurrline {
    pub cpu: Option<u32>,
    pub moves: Vec<Move>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct UciInfo {
    pub depth: Option<u32>,
    pub seldepth: Option<u32>,
    pub time: Option<Duration>,
    pub nodes: Option<u64>,
    pub pv: Option<Vec<Move>>,
    pub multipv: Option<u8>,
    pub score: Option<UciScore>,
    pub currmove: Option<Move>,
    pub currmovenumber: Option<u8>,
    pub hashfull: Option<u16>,
    pub nps: Option<u64>,
    pub tbhits: Option<u64>,
    pub sbhits: Option<u64>, // what a nice metric! I sure hope nothing would cause it to be rendered useless incredibly quickly!
    pub cpuload: Option<u16>,
    pub string: Option<String>,
    pub refutation: Option<Vec<Move>>,
    pub currline: Option<UciCurrline>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UciOptionInfo {
    Check { default: bool },
    Spin { default: i64, min: i64, max: i64 },
    Combo { default: String, labels: Vec<String> },
    Button,
    String { default: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UciRemark {
    Id(UciIdInfo),
    UciOk,
    ReadyOk,
    BestMove {
        mv: Move,
        ponder: Option<Move>,
    },
    Info(UciInfo),
    Option {
        name: String,
        info: UciOptionInfo,
    },
}

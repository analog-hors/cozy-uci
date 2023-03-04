use std::time::Duration;

use cozy_chess::{Board, Move};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UciInitPos {
    StartPos,
    Board(Board),
}

impl From<UciInitPos> for Board {
    fn from(value: UciInitPos) -> Self {
        match value {
            UciInitPos::StartPos => Board::startpos(),
            UciInitPos::Board(b) => b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct UciGoParams {
    pub searchmoves: Option<Vec<Move>>,
    pub ponder: bool,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub movestogo: Option<u32>,
    pub depth: Option<u32>,
    pub nodes: Option<u32>,
    pub mate: Option<u32>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    Position {
        init_pos: UciInitPos,
        moves: Vec<Move>,
    },
    SetOption {
        name: String,
        value: Option<String>,
    },
    UciNewGame,
    Stop,
    PonderHit,
    Quit,
    Go(UciGoParams),
}

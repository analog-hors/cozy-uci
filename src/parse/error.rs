use std::num::ParseIntError;
use std::ops::Range;

use cozy_chess::{FenParseError, MoveParseError};
use thiserror::*;

use crate::permill::PermillParseError;

#[derive(Debug, Error, Clone)]
#[error("error at {}..{}: {}", span.start, span.end, kind)]
pub struct UciParseError {
    pub span: Range<usize>,
    pub kind: UciParseErrorKind,
}

#[derive(Debug, Error, Clone)]
pub enum UciParseErrorKind {
    #[error("unexpected token `{0}`")]
    UnexpectedToken(String),
    #[error("unexpected end")]
    UnexpectedEnd,
    #[error("unterminated string")]
    UnterminatedString,
    #[error("unknown message kind {0:?}")]
    UnknownMessageKind(String),
    #[error("duplicate field {0:?}")]
    DuplicateField(&'static str),
    #[error("unknown field {0:?}")]
    UnknownField(String),
    #[error("invalid field {0:?}")]
    InvalidField(&'static str),
    #[error("failed to parse move: {0}")]
    MoveParseError(#[from] MoveParseError),
    #[error("failed to parse fen: {0}")]
    FenParseError(#[from] FenParseError),
    #[error("failed to parse int: {0}")]
    IntParseError(#[from] ParseIntError),
    #[error("failed to parse permill: {0}")]
    PermillParseError(#[from] PermillParseError),
}

impl UciParseErrorKind {
    pub(crate) fn spans(self, span: std::ops::Range<usize>) -> UciParseError {
        UciParseError { span, kind: self }
    }
}

use std::{iter::Peekable, str::FromStr};
use std::str::SplitAsciiWhitespace;
use std::num::ParseIntError;

use cozy_chess::{Move, Board, MoveParseError, FenParseError};
use thiserror::*;

use crate::permill::PermillParseError;

#[derive(Debug, Error, Clone)]
pub enum UciParseError {
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

pub struct UciTokenStream<'s> {
    str: &'s str,
    iter: Peekable<SplitAsciiWhitespace<'s>>
}

impl<'s> UciTokenStream<'s> {
    pub fn new(str: &'s str) -> Self {
        Self {
            str,
            iter: str.split_ascii_whitespace().peekable()
        }
    }

    pub fn read_token(&mut self) -> Result<&'s str, UciParseError> {
        self.iter.next().ok_or(UciParseError::UnexpectedEnd)
    }

    pub fn peek_token(&mut self) -> Result<&'s str, UciParseError> {
        self.iter.peek().copied().ok_or(UciParseError::UnexpectedEnd)
    }

    pub fn read_string(&mut self, terminates: impl Fn(Option<&str>) -> bool) -> Result<String, UciParseError> {
        let base = self.str.as_ptr() as usize;
        let start = match self.iter.peek() {
            Some(tok) => tok.as_ptr() as usize - base,
            None => self.str.len()
        };
        let mut end = start;
        while !terminates(self.iter.peek().copied()) {
            let part = self.iter.next().ok_or(UciParseError::UnterminatedString)?;
            end = part.as_ptr() as usize + part.len() - base;
        }
        Ok(self.str[start..end].to_owned())
    }

    pub fn read_type<T: FromStr>(&mut self) -> Result<T, UciParseError> where UciParseError: From<T::Err> {
        Ok(self.read_token()?.parse()?)
    }

    pub fn read_bool(&mut self) -> Result<bool, UciParseError> {
        match self.read_token()? {
            "true" => Ok(true),
            "false" => Ok(false),
            tok => Err(UciParseError::UnexpectedToken(tok.to_owned()))
        }
    }

    pub fn read_fen(&mut self, shredder: bool) -> Result<Board, UciParseError> {
        let base = self.str.as_ptr() as usize;
        let start = self.read_token()?.as_ptr() as usize - base;
        for _ in 0..4 {
            self.read_token()?;
        }
        let end_token = self.read_token()?;
        let end = end_token.as_ptr() as usize + end_token.len() - base;
        Ok(Board::from_fen(&self.str[start..end], shredder)?)
    }

    pub fn read_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        while let Ok(mv) = self.peek_token().and_then(|tok| Ok(tok.parse()?)) {
            let _ = self.read_token();
            moves.push(mv);
        }
        moves
    }

    pub fn expect_token(&mut self, expected: &str) -> Result<(), UciParseError> {
        let tok = self.read_token()?;
        if tok != expected {
            return Err(UciParseError::UnexpectedToken(tok.to_owned()));
        }
        Ok(())
    }

    pub fn expect_end(&mut self) -> Result<(), UciParseError> {
        if let Some(tok) = self.iter.next() {
            return Err(UciParseError::UnexpectedToken(tok.to_owned()));
        }
        Ok(())
    }
}

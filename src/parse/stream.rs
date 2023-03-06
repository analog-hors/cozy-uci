use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;
use std::str::SplitAsciiWhitespace;

use cozy_chess::{Board, Move};

use super::error::{UciParseError, UciParseErrorKind};
use UciParseErrorKind::*;

pub struct UciTokenStream<'s> {
    str: &'s str,
    iter: Peekable<SplitAsciiWhitespace<'s>>,
}

impl<'s> UciTokenStream<'s> {
    pub fn new(str: &'s str) -> Self {
        Self {
            str,
            iter: str.split_ascii_whitespace().peekable(),
        }
    }

    pub fn read_token(&mut self) -> Result<(&'s str, Range<usize>), UciParseError> {
        let span = self.curr_tok_span();
        match self.iter.next() {
            Some(tok) => Ok((tok, span)),
            None => Err(UnexpectedEnd.spans(span)),
        }
    }

    pub fn peek_token(&mut self) -> Result<(&'s str, Range<usize>), UciParseError> {
        let span = self.curr_tok_span();
        match self.iter.peek().copied() {
            Some(tok) => Ok((tok, span)),
            None => Err(UnexpectedEnd.spans(span)),
        }
    }

    pub fn read_string(
        &mut self,
        terminates: impl Fn(Option<&str>) -> bool,
    ) -> Result<String, UciParseError> {
        let base = self.str.as_ptr() as usize;
        let start = match self.iter.peek() {
            Some(tok) => tok.as_ptr() as usize - base,
            None => self.str.len(),
        };
        let mut end = start;
        while !terminates(self.iter.peek().copied()) {
            let part = self
                .iter
                .next()
                .ok_or(UnterminatedString.spans(start..self.str.len()))?;
            end = part.as_ptr() as usize + part.len() - base;
        }
        Ok(self.str[start..end].to_owned())
    }

    pub fn read_type<T: FromStr>(&mut self) -> Result<T, UciParseError>
    where
        UciParseErrorKind: From<T::Err>,
    {
        let (tok, span) = self.read_token()?;
        tok.parse()
            .map_err(|e| UciParseErrorKind::from(e).spans(span))
    }

    pub fn read_bool(&mut self, true_tok: &str, false_tok: &str) -> Result<bool, UciParseError> {
        let (tok, span) = self.read_token()?;
        match tok {
            tok if tok == true_tok => Ok(true),
            tok if tok == false_tok => Ok(false),
            tok => Err(UnexpectedToken(tok.to_owned()).spans(span)),
        }
    }

    pub fn read_fen(&mut self, shredder: bool) -> Result<Board, UciParseError> {
        let start = self.read_token()?.1.start;
        for _ in 0..4 {
            self.read_token()?;
        }
        let end = self.read_token()?.1.end;
        let board = Board::from_fen(&self.str[start..end], shredder)
            .map_err(|e| FenParseError(e).spans(start..end))?;
        Ok(board)
    }

    pub fn read_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        while let Ok(Ok(mv)) = self.peek_token().map(|(tok, _)| tok.parse()) {
            let _ = self.read_token();
            moves.push(mv);
        }
        moves
    }

    pub fn expect_token(&mut self, expected: &str) -> Result<(), UciParseError> {
        let (tok, span) = self.read_token()?;
        if tok != expected {
            return Err(UnexpectedToken(tok.to_owned()).spans(span));
        }
        Ok(())
    }

    pub fn expect_end(&mut self) -> Result<(), UciParseError> {
        let span = self.curr_tok_span();
        if let Some(tok) = self.iter.next() {
            return Err(UnexpectedToken(tok.to_owned()).spans(span));
        }
        Ok(())
    }

    pub fn curr_tok_span(&mut self) -> std::ops::Range<usize> {
        let base = self.str.as_ptr() as usize;
        let (start, len) = match self.iter.peek() {
            Some(tok) => (tok.as_ptr() as usize - base, tok.len()),
            None => (self.str.len(), 0),
        };
        start..start + len
    }
}

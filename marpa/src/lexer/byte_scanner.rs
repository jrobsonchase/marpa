use crate::lexer::token::Token;
use crate::thin::Symbol;
use std::fmt;
use std::str;

#[derive(Default, PartialEq, Eq, PartialOrd, Debug, Copy, Clone)]
pub struct ByteToken(u8);

impl fmt::Display for ByteToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: [u8; 1] = [self.0];
        match str::from_utf8(&a) {
            Err(_) => write!(f, "{:x}", self.0),
            Ok(s) => write!(f, "'{}'", s),
        }
    }
}

impl From<(Symbol, i32)> for ByteToken {
    fn from((sym, _): (Symbol, i32)) -> Self {
        ByteToken(sym as u8)
    }
}

impl Token for ByteToken {
    fn sym(&self) -> Symbol {
        i32::from(self.0)
    }

    fn value(&self) -> i32 {
        i32::from(self.0) + 1
    }
}

impl ::std::ops::Deref for ByteToken {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ByteScanner<R: ::std::io::Read> {
    rd: R,
    buffer: Vec<u8>,
    idx: usize,
    end: usize,
}

impl<R: ::std::io::Read> ByteScanner<R> {
    pub fn new(rd: R) -> ByteScanner<R> {
        ByteScanner::with_capacity(rd, 256)
    }

    pub fn with_capacity(rd: R, cap: usize) -> ByteScanner<R> {
        ByteScanner {
            rd,
            buffer: vec![0; cap],
            idx: 0,
            end: 0,
        }
    }

    fn fill_buffer(&mut self) -> ::std::io::Result<usize> {
        self.rd.read(&mut self.buffer)
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.idx >= self.end {
            self.idx = 0;
            match self.fill_buffer() {
                Ok(size) => {
                    if size == 0 {
                        return None;
                    } else {
                        self.end = size
                    }
                }
                Err(_) => return None,
            }
        }

        self.idx += 1;
        Some(self.buffer[self.idx - 1])
    }
}

impl<R: ::std::io::Read> Iterator for ByteScanner<R> {
    type Item = ByteToken;

    fn next(&mut self) -> Option<ByteToken> {
        match self.read_byte() {
            Some(byte) => Some(ByteToken(byte)),
            None => None,
        }
    }
}

// impl<R: ::std::io::Read> TokenSource for ByteScanner<R> {}

#[cfg(test)]
mod tests {
    use super::super::token_source::TokenSource;
    use super::ByteScanner;
    use super::ByteToken;
    use std::io::Cursor;

    #[test]
    fn test_byte_scanner() {
        let input = Cursor::new("Hello, world!");
        let scanner = ByteScanner::new(input);
        must_compile(&scanner);

        let toks: Vec<ByteToken> = scanner.collect();
        assert!(
            toks == vec![
                ByteToken(72),
                ByteToken(101),
                ByteToken(108),
                ByteToken(108),
                ByteToken(111),
                ByteToken(44),
                ByteToken(32),
                ByteToken(119),
                ByteToken(111),
                ByteToken(114),
                ByteToken(108),
                ByteToken(100),
                ByteToken(33),
            ]
        );
    }

    fn must_compile<T: TokenSource<ByteToken>>(_: &T) {}
}

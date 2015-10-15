use lexer::token::Token;

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
            rd: rd,
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
                Ok(size) => if size == 0 { return None } else { self.end = size },
                Err(_) => return None,
            }
        }

        self.idx += 1;
        Some(self.buffer[self.idx-1])
    }
}

impl<R: ::std::io::Read> Iterator for ByteScanner<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.read_byte() {
            Some(byte) => Some(Token::new(byte as i32, (byte+1) as i32)),
            None => None,
        }
    }
}

// impl<R: ::std::io::Read> TokenSource for ByteScanner<R> {}

#[cfg(test)]
mod tests {
    use super::ByteScanner;
    use super::super::token::Token;
    use super::super::token_source::TokenSource;
    use std::io::Cursor;

    #[test]
    fn test_byte_scanner() {
        let input = Cursor::new("Hello, world!");
        let scanner = ByteScanner::new(input);
        must_compile(&scanner);

        let toks: Vec<Token> = scanner.collect();
        assert!(toks == vec![
            Token { ty: 72, val: 73 },
            Token { ty: 101, val: 102 },
            Token { ty: 108, val: 109 },
            Token { ty: 108, val: 109 },
            Token { ty: 111, val: 112 },
            Token { ty: 44, val: 45 },
            Token { ty: 32, val: 33 },
            Token { ty: 119, val: 120 },
            Token { ty: 111, val: 112 },
            Token { ty: 114, val: 115 },
            Token { ty: 108, val: 109 },
            Token { ty: 100, val: 101 },
            Token { ty: 33, val: 34 },
            ]);
    }

    fn must_compile<T: TokenSource>(_: &T){}
}

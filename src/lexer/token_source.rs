pub trait TokenSource {
    fn next_token(&mut self) -> Option<super::token::Token>;
}

impl<I> TokenSource for I where I: Iterator<Item=super::token::Token> {
    fn next_token(&mut self) -> Option<super::token::Token> {
        self.next()
    }
}

use super::token::Token;

pub trait TokenSource<T>: Iterator<Item = T>
where
    T: Token,
{
}
impl<T, U> TokenSource<U> for T
where
    T: Iterator<Item = U>,
    U: Token,
{
}

// pub struct VecTokenSource {
//     tokens: Vec<Token>,
//     idx: usize,
// }

// impl TokenSource for VecTokenSource {
//     fn next_token(&mut self) -> Option<Token> {
//         self.idx += 1;
//         self.tokens.get(self.idx - 1).map(|t| *t)
//     }
// }

// impl<'a> From<&'a [Token]> for VecTokenSource {
//     fn from(other: &'a [Token]) -> VecTokenSource {
//         VecTokenSource{ tokens: other.into(), idx: 0 }
//     }
// }

// impl From<Vec<Token>> for VecTokenSource {
//     fn from(other: Vec<Token>) -> VecTokenSource {
//         VecTokenSource{ tokens: other, idx: 0 }
//     }
// }

pub type MarpaResult<T> = Result<T, String>;

pub fn err<S: Into<String>, T>(msg: S) -> MarpaResult<T> {
    Err(msg.into())
}

pub fn err_code<T>(code: i32) -> MarpaResult<T> {
    Err(format!("error code when creating grammar: {}", code))
}

pub const ErrNotOk: &'static str = "marpa is not in an ok state";
pub const ErrNoSym: &'static str = "no such symbol";
pub const ErrNotPrecomputed: &'static str = "grammar is not precomputed";

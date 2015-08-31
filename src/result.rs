use desc;

pub type MarpaResult<T> = Result<T, String>;

pub fn err<S: Into<String>, T>(msg: S) -> MarpaResult<T> {
    Err(msg.into())
}

pub fn err_code<T>(code: i32) -> MarpaResult<T> {
    Err(desc::err_desc(code as usize).into())
}

use desc;

pub type MarpaResult<T> = Result<T, String>;

pub fn err<S: Into<String>, T>(msg: S) -> MarpaResult<T> {
    Err(msg.into())
}

pub fn err_code<T>(code: i32) -> MarpaResult<T> {
    match code {
        code if code >= 0 => Err(desc::err_desc(code as usize).into()),
        code => Err(format!("undefined error code: {}", code)),
    }
}

pub fn err_nosym<T>() -> MarpaResult<T> {
    err_code(90)
}

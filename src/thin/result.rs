#![allow(unused)]

use thin::desc;
use std::result;

pub type Result<T> = result::Result<T, String>;

pub fn err<S: Into<String>, T>(msg: S) -> Result<T> {
    Err(msg.into())
}

pub fn err_code<T>(code: i32) -> Result<T> {
    match code {
        code if code >= 0 => Err(desc::err_desc(code as usize).into()),
        code => Err(format!("undefined error code: {}", code)),
    }
}

pub fn err_nosym<T>() -> Result<T> {
    err_code(90)
}

pub fn err_norule<T>() -> Result<T> {
    err_code(89)
}

pub fn err_notaseq<T>() -> Result<T> {
    err_code(99)
}

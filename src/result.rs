#![allow(unused)]

use error::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub fn err<T>(msg: &str) -> Result<T> {
    Err(msg.into())
}

pub fn err_code<T>(code: i32) -> Result<T> {
    Err(code.into())
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

pub fn err_rnotstarted<T>() -> Result<T> {
    err_code(61)
}

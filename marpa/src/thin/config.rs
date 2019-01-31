use std::{mem, ptr};

use libmarpa_sys::*;

use crate::result::*;

pub struct Config {
    internal: Marpa_Config,
}

impl Default for Config {
    fn default() -> Self {
        let internal: Marpa_Config = unsafe {
            let mut cfg: Marpa_Config = mem::zeroed();
            assert!(marpa_c_init(&mut cfg) == MARPA_ERR_NONE as _);
            cfg
        };
        Config { internal }
    }
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn internal(&self) -> Marpa_Config {
        self.internal
    }

    pub fn error(&mut self) -> Result<()> {
        unsafe {
            match marpa_c_error(&mut self.internal, ptr::null_mut()) as _ {
                0 => Ok(()),
                err if err > 0 => err_code(err),
                _ => err("error creating grammar"),
            }
        }
    }
}

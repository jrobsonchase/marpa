use thin::libmarpa_sys::*;

use result::*;
use std::ptr;

#[derive(Default)]
pub struct Config {
    internal: Marpa_Config,
}

impl Config {
    pub fn new() -> Config {
        let mut cfg = Config::default();

        assert!(cfg.init() == MARPA_ERR_NONE);

        cfg
    }

    pub fn internal(&self) -> Marpa_Config {
        self.internal
    }

    fn init(&mut self) -> Marpa_Error_Code {
        unsafe { marpa_c_init(&mut self.internal) }
    }

    pub fn error(&mut self) -> Result<()> {
        unsafe {
            match marpa_c_error(&mut self.internal, ptr::null_mut()) {
                0 => Ok(()),
                err if err > 0 => err_code(err),
                _ => err("error creating grammar"),
            }
        }
    }
}

use thin::libmarpa_sys::*;

use std::ptr;
use thin::result::*;

pub struct Config {
    internal: Marpa_Config,
}

pub fn internal(cfg: &Config) -> Marpa_Config {
    cfg.internal
}

impl Config {
    pub fn new() -> Config {
        let mut cfg = Config { internal: Marpa_Config::default() };

        assert!(cfg.init() == MARPA_ERR_NONE);

        cfg
    }

    fn init(&mut self) -> Marpa_Error_Code {
        unsafe {
            marpa_c_init(&mut self.internal)
        }
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

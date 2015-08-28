use libmarpa_sys::{Marpa_Config, marpa_c_init, marpa_c_error, Marpa_Error_Code};

use std::ptr;

pub struct Config {
    pub internal: *mut Marpa_Config,
}

impl Config {
    pub fn new() -> Config {
        let mut cfg = Config { internal: &mut Marpa_Config::default() };

        cfg.init();

        cfg
    }

    fn init(&mut self) {
        unsafe {
            marpa_c_init(self.internal);
        }
    }

    pub fn error(&self) -> i32 {
        unsafe {
            marpa_c_error(self.internal, ptr::null_mut())
        }
    }
}

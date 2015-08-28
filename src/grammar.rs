use libmarpa_sys::{Marpa_Grammar, marpa_g_new};

use config::Config;

pub struct Grammar {
    pub internal: Marpa_Grammar,
}

impl Grammar {
    pub fn new(cfg: &Config) -> Grammar {
        unsafe {
            let c_grammar = marpa_g_new(cfg.internal);

            match cfg.error() {
                0 => Grammar { internal: c_grammar },
                29 => panic!("marpa is not in an ok state"),
                err => panic!("error code when creating grammar: {}", err),
            }
        }
    }
}

#[test]
fn create_grammar() {
    let cfg = Config::new();
    let grammar = Grammar::new(&cfg);
}

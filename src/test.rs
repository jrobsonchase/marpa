use ::{Marpa_Config,marpa_c_init,marpa_g_new,marpa_c_error,MARPA_ERR_NONE};

use std::ptr;

#[test]
fn create_grammar() {
    unsafe {
        let mut a = Marpa_Config::default();
        let b = marpa_c_init(&mut a);
        let c = marpa_g_new(&mut a);
        assert!(b == MARPA_ERR_NONE);
        assert!(marpa_c_error(&mut a, ptr::null_mut()) == MARPA_ERR_NONE);
    }
}

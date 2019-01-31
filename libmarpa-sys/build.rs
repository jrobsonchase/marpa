use std::{env, path::PathBuf};

use autotools::Config;

const LIBMARPA_VERSION: &'static str = "8.3.0";

fn main() {
    let current_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libmarpa_dir = current_dir.join(format!("libmarpa-{}", LIBMARPA_VERSION));
    env::set_var("CFLAGS", format!("{} -fPIC", env::var("CFLAGS").unwrap_or_default())); // workaround for .cflag("-fPIC") ?
    let dst = Config::new(libmarpa_dir).build().join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=marpa");
}

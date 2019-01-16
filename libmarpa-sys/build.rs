use std::process::Command;
use std::env;

fn main() {
    let dir = env::current_dir().unwrap();

    let libmarpa_version = "8.3.0";
    let libmarpa_dir = format!("{}/libmarpa-{}", dir.display(), libmarpa_version);
    let libmarpa_tarball = format!("{}.tar.gz", libmarpa_dir);

    Command::new("tar").arg("xf").arg(&libmarpa_tarball).status().unwrap();
    Command::new("./configure").current_dir(&libmarpa_dir)
        .env("CFLAGS", "-fPIC")
        .status().unwrap();
    Command::new("make").current_dir(&libmarpa_dir)
        .status().unwrap();
    println!("cargo:rustc-link-search=native={}/.libs", libmarpa_dir);
    println!("cargo:rustc-link-lib=static=marpa");
}

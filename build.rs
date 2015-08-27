use std::process::Command;

fn main() {
    let libmarpa_version = "8.3.0";
    let libmarpa_dir = format!("libmarpa-{}", libmarpa_version);
    let libmarpa_tarball = format!("{}.tar.gz", libmarpa_dir);

    Command::new("tar").arg("xf").arg(&libmarpa_tarball).status().unwrap();
    Command::new("./configure").current_dir(&libmarpa_dir).status().unwrap();
    Command::new("make").current_dir(&libmarpa_dir).status().unwrap();

    println!("cargo:rustc-link-search=native={}/.libs", libmarpa_dir);
    println!("cargo:rustc-link-lib=static=marpa");
}

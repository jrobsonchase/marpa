use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

const LIBMARPA_VERSION: &str = "8.3.0";

fn main() {
    let crate_root: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    let libmarpa_name = format!("libmarpa-{}", LIBMARPA_VERSION);
    let libmarpa_tarball = crate_root.join(format!("{}.tar.gz", libmarpa_name));
    let libmarpa_out_dir = out_dir.join(&libmarpa_name);

    extract_tar_gz(&libmarpa_tarball, &out_dir).unwrap();

    let configure_status = Command::new(libmarpa_out_dir.join("configure"))
        .current_dir(&libmarpa_out_dir)
        .env("CFLAGS", "-fPIC")
        .status()
        .expect("configure");
    assert!(configure_status.success(), "Configure command failed");

    let make_status = Command::new("make").current_dir(&libmarpa_out_dir).status().expect("make");
    assert!(make_status.success(), "Make command failed");

    println!("cargo:rustc-link-search=native={}/.libs", libmarpa_out_dir.display());
    println!("cargo:rustc-link-lib=static=marpa");
}

fn extract_tar_gz<P, Q>(path: P, out_dir: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    Ok(archive.unpack(out_dir)?)
}

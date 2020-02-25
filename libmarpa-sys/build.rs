use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

use bindgen::builder;

use lazy_static::lazy_static;

const LIBMARPA_VERSION: &str = "8.6.2";

lazy_static! {
    static ref CRATE_ROOT: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    static ref OUT_DIR: PathBuf = env::var("OUT_DIR").unwrap().into();
}

fn main() {
    let path = extract_libmarpa(LIBMARPA_VERSION).unwrap();
    build_libmarpa(&path).unwrap();
    run_bindgen(&path).unwrap();
}

// extract a tarball at `path` into `out_dir`
//
// borrowed from the rust cookbook: https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
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

// extract a libmarpa tarball with the provided version. Returns the extracted
// libmarpa directory.
//
// Assumes that there's a libmarpa-VERSION.tar.gz in the crate root
fn extract_libmarpa(version: &'static str) -> io::Result<PathBuf> {
    let libmarpa_name: String = format!("libmarpa-{}", version);
    let libmarpa_tarball = CRATE_ROOT.join(format!("{}.tar.gz", libmarpa_name));
    println!("cargo:rerun-if-changed={}", libmarpa_tarball.display());
    extract_tar_gz(libmarpa_tarball, &*OUT_DIR)?;
    Ok(OUT_DIR.join(libmarpa_name))
}

// build the extracted libmarpa at the given path
fn build_libmarpa<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let configure_status = Command::new(path.as_ref().join("configure"))
        .current_dir(&path)
        .env("CFLAGS", "-fPIC -O3")
        .status()?;
    if !configure_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "configure failed"));
    }

    let make_status = Command::new("make").current_dir(&path).status()?;
    if !make_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    println!("cargo:rustc-link-search=native={}/.libs", path.as_ref().display());
    println!("cargo:rustc-link-lib=static=marpa");
    Ok(())
}

// run bindgen on the extracted libmarpa at the given path
//
// assumes that there's a 'marpa.h' in the provided directory
fn run_bindgen<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let bindings = builder()
        .header(format!("{}", path.as_ref().join("marpa.h").display()))
        .blacklist_type("max_align_t")
        .generate()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to generate bindings"))?;

    bindings.write_to_file(OUT_DIR.join("raw.rs"))
}

#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum SearchLibKind {
    Dependency,
    Crate,
    Native,
    Framework,
    All,
}

#[derive(Copy, Clone, Debug)]
pub enum LibKind {
    Dylib,
    Static,
    Framework,
}

#[derive(Copy, Clone, Debug)]
pub enum LinkArgTarget {
    Bin(&'static str),
    Bins,
    Tests,
    Examples,
    Benches,
    Cdylib,
}

/// Tells Cargo when to re-run the script.
pub fn rerun_if_changed<P: AsRef<Path>>(p: P) {
    println!("cargo:rerun-if-changed={}", p.as_ref().display());
}

/// Tells Cargo when to re-run the script.
pub fn rerun_if_env_changed(env_var: &str) {
    println!("cargo:rerun-if-env-changed={}", env_var);
}

/// Sets an environment variable which can be accessed using var! macro
pub fn rustc_env(var: &str, value: &str) {
    println!("cargo:rustc-env={}={}", var, value);
}

/// Enables compile-time cfg settings.
pub fn rustc_cfg(key: &str, value: Option<&str>) {
    let value = if let Some(v) = value {
        format!("{}={}", key, v)
    } else {
        key.to_string()
    };
    println!("cargo:rustc-cfg={}", value);
}

/// Passes custom flags to a linker for benchmarks, binaries, cdylib crates, examples, and tests.
pub fn rustc_link_arg<T: Into<Option<LinkArgTarget>>>(arg: &str, target: T) {
    let mut is_cdylib = false;
    let target = if let Some(t) = target.into() {
        match t {
            LinkArgTarget::Bin(s) => format!("-bin={}", &s),
            LinkArgTarget::Bins => "-bins".to_string(),
            LinkArgTarget::Tests => "-tests".to_string(),
            LinkArgTarget::Benches => "-benches".to_string(),
            LinkArgTarget::Examples => "-examples".to_string(),
            LinkArgTarget::Cdylib => {
                is_cdylib = true;
                String::new()
            }
        }
    } else {
        String::new()
    };
    println!(
        "cargo:rustc-{}link-arg{}={}",
        if is_cdylib { "cdylib-" } else { "" },
        target,
        arg
    );
}

/// Adds to the library search path.
pub fn rustc_link_search<P: AsRef<Path>, T: Into<Option<SearchLibKind>>>(p: P, kind: T) {
    let kind = if let Some(k) = kind.into() {
        k
    } else {
        SearchLibKind::All
    };
    println!(
        "cargo:rustc-link-search={}={}",
        &format!("{:?}", kind).to_lowercase(),
        p.as_ref().display()
    );
}

/// Adds a library to link
pub fn rustc_link_lib<T: Into<Option<LibKind>>>(lib: &str, kind: T) {
    let lib = if let Some(k) = kind.into() {
        format!("{}={}", format!("{:?}", k).to_lowercase(), lib)
    } else {
        lib.to_string()
    };
    println!("cargo:rustc-link-lib={}", lib);
}

/// Metadata, used by links scripts.
pub fn metadata_set(key: &str, val: &str) {
    println!("cargo:{}={}", key, val);
}

/// Displays a warning on the terminal.
pub fn warning(msg: &str) {
    println!("cargo:warning={}", msg);
}

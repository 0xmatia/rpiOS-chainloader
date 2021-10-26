use std::env;

fn main() {
    // because this script is called from the Makefile, the
    // LINKER_FILE variables, which is exported on line 43, 
    // is available as an environment variable
    let linker_file = env::var("LINKER_FILE").unwrap_or_default();

    // not 100% why, but this tell rustc to recombile if 
    // either the linker script has changed or the build script itself
    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
}
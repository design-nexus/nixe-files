// SPDX-License-Identifier: MIT
fn main() {
    println!("cargo:rerun-if-changed=../src/rar_extraction.rs");
    let wire = std::fs::read_to_string("../src/rar_extraction.rs").expect("shared MIT wire protocol")
        .replace("//!", "//")
        .replace("#[cfg(test)]\nmod tests;", "");
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("build directory"));
    std::fs::write(output.join("rar_wire.rs"), wire).expect("stage shared wire protocol");
}

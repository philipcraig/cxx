fn main() {
    cxx_build::bridge("src/main.rs")
        .file("../demo-cxx/demo.cc")
        .flag_if_supported("-std=c++14") // gcc/clang
        .flag_if_supported("/std:c++14") // x86_64-pc-windows-msvc
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=../demo-cxx/demo.h");
    println!("cargo:rerun-if-changed=../demo-cxx/demo.cc");
}

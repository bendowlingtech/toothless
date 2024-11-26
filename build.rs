

fn main() {

    println!("cargo:rerun-if-changed=c/*.c");
    println!("cargo:rerun-if-changed=c/*.h");

    cc::Build::new()
        .include("c")
        .files([
            "c/php_wrapper.c",
        ])
        .compile("toothless");

    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/c");
    println!("cargo:rustc-link-lib=static=frankenphp");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=php7.4");
        println!("cargo:rustc-link-lib=dylib=php_embed");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=php");
        println!("cargo:rustc-link-lib=dylib=php_embed");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=php");
        println!("cargo:rustc-link-lib=php_embed");
    }
}
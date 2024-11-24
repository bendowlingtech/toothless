

fn main() {

    println!("cargo:rerun-if-changed=c/*.c");
    println!("cargo:rerun-if-changed=c/*.h");

    cc::Build::new()
        .file("c/php_wrapper.c")
        .compile("toothless");
}
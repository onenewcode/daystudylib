// build.rs
fn main() {
    println!("cargo:rustc-link-search=native=/home/ztf/llama.cpp/build/bin");
    println!("cargo:rustc-link-lib=dylib=ggml");
    println!("cargo:rustc-link-lib=dylib=ggml-cpu");
}

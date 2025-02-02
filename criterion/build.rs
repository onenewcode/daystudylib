// build.rs
fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=ggml");
        println!("cargo:rustc-link-lib=dylib=ggml-cpu");
        println!("cargo:rustc-link-search=native=/home/ztf/llama.cpp/build/bin");
    }
}

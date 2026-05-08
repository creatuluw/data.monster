fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-arg=/STACK:33554432");
    }
    tauri_build::build()
}

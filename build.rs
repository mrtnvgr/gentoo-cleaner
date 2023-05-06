fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(windows)]
    {
        panic!("This program is not intended to be used on Windows.");
    }
}

fn main() {
    // Library paths
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::current_dir().unwrap().join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=gapcom");
}

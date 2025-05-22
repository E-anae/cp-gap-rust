fn main() {
    cc::Build
        ::new()
        .file("sys.c") // Chemin vers syscalls.c
        .compile("syscalls");
    // Library paths
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::current_dir().unwrap().join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=gapcom");

    // For embedded targets, don't link standard libc
    let target = std::env::var("TARGET").unwrap();
    if target.contains("thumbv7em-none-eabihf") {
        println!("cargo:rustc-link-arg=-nostartfiles");
    }
}

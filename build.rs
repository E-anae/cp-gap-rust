use cc::Build;

fn main() {
    // Library paths
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::current_dir().unwrap().join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=gapcom");

    // For embedded targets
    let target = std::env::var("TARGET").unwrap();
    if target.contains("thumbv7em-none-eabihf") {
        println!("cargo:rustc-link-arg=-nostartfiles");
        println!("cargo:rustc-link-arg=-mthumb");
        println!("cargo:rustc-link-arg=-mcpu=cortex-m4");
        println!("cargo:rustc-link-arg=-mfloat-abi=hard");
        println!("cargo:rustc-link-arg=-mfpu=fpv4-sp-d16");
        println!("cargo:rustc-link-arg=-nostdlib");
    }
}

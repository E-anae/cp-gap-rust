fn main() {
    let current_dir = std::env::current_dir().unwrap();

    // // Set up library paths and linking
    println!("cargo:rustc-link-search=native={}", current_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=gapcom");
    // println!("cargo:rustc-link-search=native=/usr/lib/arm-none-eabi/lib");
    // println!("cargo:rustc-link-search=native=/usr/arm-none-eabi/lib");
    // println!("cargo:rustc-link-lib=static=c");
    // println!("cargo:rustc-link-lib=static=m");
    // println!("cargo:rustc-link-lib=static=nosys");

    // let target = std::env::var("TARGET").unwrap_or_else(|_| "".to_string());
    // let is_arm = target.contains("thumb") || target.contains("arm");

    // if is_arm {
    //     // Link against newlib for embedded targets
    // }

    // let mut builder = bindgen::Builder::default();

    // // Add the project's include directories
    // builder = builder
    //     .clang_arg(format!("-I{}", current_dir.join("Inc").display()))
    //     .clang_arg(format!("-I{}", current_dir.join("Inc/gapcom").display()))
    //     .clang_arg(format!("-I{}", current_dir.join("Inc/gapcom/gapcom").display()))
    //     .clang_arg(format!("-I{}", current_dir.join("Inc/nanopb").display()))
    //     .clang_arg(format!("-I{}", current_dir.join("Inc/TinyFrame").display()));

    // if is_arm {
    //     // Add ARM gcc toolchain paths and configure for embedded target
    //     builder = builder
    //         .clang_arg("--target=arm-none-eabi")
    //         .clang_arg("-I/usr/arm-none-eabi/include")
    //         .clang_arg("-I/usr/lib/arm-none-eabi/include")
    //         .clang_arg("-I/usr/lib/gcc/arm-none-eabi/13.2.1/include")
    //         .use_core()
    //         .ctypes_prefix("cty")
    //         .clang_arg("--sysroot=/usr/arm-none-eabi")
    //         // Allow common types that are safe to use
    //         .allowlist_type("u?int[0-9]+_t")
    //         .allowlist_type("size_t")
    //         .allowlist_type("ssize_t")
    //         // Allow our gapcom types
    //         .allowlist_type("gapcom_.*")
    //         .allowlist_function("gapcom_.*");
    // } else {
    //     builder = builder
    //         .clang_arg("-I/usr/include")
    //         .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/11/include")
    //         .clang_arg("-I/usr/local/include");
    // }

    // let bindings = builder
    //     .header("Inc/gapcom/gapcom/gapcom.h")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .use_core()
    //     .generate()
    //     .expect("Failed to generate bindings");

    // let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // bindings.write_to_file(out_path.join("bindings.rs")).expect("Failed to write bindings");
}

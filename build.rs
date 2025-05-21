fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=libgapcom");

    let bindings = bindgen::Builder::default()
        .header("Inc/gapcom/gapcom/gapcom.h")
	.clang_arg("-I/usr/include")
        .generate()
        .expect("Échec de la génération des bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Échec de l'écriture des bindings");
}

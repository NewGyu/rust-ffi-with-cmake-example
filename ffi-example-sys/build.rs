use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let em_cmake = PathBuf::from(env::var("EMSDK").unwrap()).join("upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake");
    let dst = Config::new("src/c")
        .define("CMAKE_BUILD_TYPE", "MinSizeRel")
        .define("CMAKE_TOOLCHAIN_FILE", em_cmake)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=string");
    println!("cargo:rustc-link-lib=static=array");

    let bindings = bindgen::Builder::default()
        .header("src/c/all.h")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

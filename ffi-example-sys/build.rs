use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut cm = Config::new("src/c");
    cm.define("CMAKE_BUILD_TYPE", "MinSizeRel");
    if cfg!(target_arch = "wawm32") {
        let em_cmake = PathBuf::from(env!("EMSDK"))
            .join("upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake");
        cm.define("CMAKE_TOOLCHAIN_FILE", em_cmake);
    }
    let dst = &cm.build();

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

use std::env;
use std::path::PathBuf;

fn set_include_path(bindings: bindgen::Builder) -> bindgen::Builder {
    match std::env::consts::OS {
        // TODO: be more specific that this is for the Miyoo Mini. In theory we can also build for generic linux.
        "linux" => bindings
            .clang_arg("-I/opt/miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/include")
            .clang_arg("-I/opt/miyoomini-toolchain/lib/gcc/arm-linux-gnueabihf/8.3.0/include"),
        "macos" => bindings.clang_arg("-I/opt/homebrew/include"),
        _ => {
            panic!("Unsupported OS");
        }
    }
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/opt/homebrew/lib");

    // Tell cargo to tell rustc to link SDL libs.
    println!("cargo:rustc-link-lib=SDL");
    println!("cargo:rustc-link-lib=SDL_image");
    println!("cargo:rustc-link-lib=SDL_ttf");

    let bindings = set_include_path(bindgen::Builder::default())
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

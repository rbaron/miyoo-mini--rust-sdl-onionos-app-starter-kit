use std::env;
use std::path::PathBuf;

// fn make_bindings() -> bindgen::Bindings {
//     match std::env::consts::OS {
//         "linux" => bindgen::Builder::default()
//             .header("wrapper.h")
//             .clang_arg("-I/opt/miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/include")
//             .clang_arg("-I/opt/miyoomini-toolchain/lib/gcc/arm-linux-gnueabihf/8.3.0/include")
//             .generate()
//             .expect("Unable to generate bindings"),
//         "macos" => bindgen::Builder::default()
//             .header("wrapper.h")
//             .clang_arg("-I/opt/homebrew/include")
//             .generate()
//             .expect("Unable to generate bindings"),
//         _ => {
//             panic!("Unsupported OS");
//         }
//     }
// }

fn set_include_path(bindings: bindgen::Builder) -> bindgen::Builder {
    match std::env::consts::OS {
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

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=SDL");
    println!("cargo:rustc-link-lib=SDL_image");
    println!("cargo:rustc-link-lib=SDL_ttf");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = set_include_path(bindgen::Builder::default())
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // .clang_arg("-I/opt/miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/include")
        // .clang_arg("-I/opt/miyoomini-toolchain/lib/gcc/arm-linux-gnueabihf/8.3.0/include")
        // .clang_arg("-I/opt/homebrew/include")
        // .clang_arg("-I/opt/miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/include/linux/")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
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

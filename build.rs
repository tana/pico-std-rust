// Build procedure is based on esp-idf-sys: https://github.com/esp-rs/esp-idf-sys/blob/155299bde700905fd2ddb040d7a13fb73559ac68/build/native/cargo_driver.rs

use embuild::build::LinkArgsBuilder;
use embuild::cargo;
use embuild::cmake::file_api::{ObjKind, Query};
use embuild::cmake::Config;

fn main() {
    let cmake_build_dir = cargo::out_dir().join("build");

    // Set CMake to output API files https://cmake.org/cmake/help/git-stage/manual/cmake-file-api.7.html
    let query = Query::new(
        &cmake_build_dir,
        "cargo",
        &[ObjKind::Codemodel],
    )
    .unwrap();

    // Build C part
    Config::new("c")
        .target("thumbv6m-none-eabi")
        .define("CMAKE_SYSTEM_NAME", "")
        .generator("Ninja")
        .build_target("exe")
        .build();

    // Retrieve information from CMake API files
    let replies = query.get_replies().unwrap();
    let codemodel = replies.get_codemodel().unwrap();
    let exe_target = codemodel
        .into_first_conf()
        .get_target("exe")
        .unwrap()
        .unwrap();
    let link = exe_target.link.unwrap();

    let link_args = LinkArgsBuilder::try_from(&link)
        .unwrap()
        .force_ldproxy(true)
        .linker("arm-none-eabi-gcc")
        .working_directory(&cmake_build_dir)
        .build()
        .unwrap();
    link_args.output();

    // Wrap main function with initialization routine
    println!("cargo:rustc-link-arg=-Wl,--wrap=main");
}

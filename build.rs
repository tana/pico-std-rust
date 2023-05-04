fn main() {
    let dst = cmake::Config::new("c")
        .target("thumbv6m-none-eabi")
        .define("CMAKE_SYSTEM_NAME", "")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=pico-std-rust-c");
}

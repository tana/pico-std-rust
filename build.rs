fn main() {
    let dst = cmake::Config::new("c")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=pico-std-rust-c");
}

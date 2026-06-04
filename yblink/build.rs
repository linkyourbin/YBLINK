use std::env;

fn main() {
    let encoded = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let parent_config_has_linker_scripts = encoded
        .split('\x1f')
        .any(|flag| flag.contains("-Tmemory.x"));

    if !parent_config_has_linker_scripts {
        println!("cargo:rustc-link-arg=-Tmemory.x");
        println!("cargo:rustc-link-arg=-Tlink.x");
        println!("cargo:rustc-link-arg=-Tboot_header.x");
        println!("cargo:rustc-link-arg=-Tnoncacheable.x");
        println!("cargo:rustc-link-arg=-Tfast.x");
        println!("cargo:rustc-link-arg=-nmagic");
    }

    println!("cargo:rerun-if-changed=../memory.x");
    println!("cargo:rerun-if-changed=../boot_header.x");
    println!("cargo:rerun-if-changed=../noncacheable.x");
    println!("cargo:rerun-if-changed=../fast.x");
}

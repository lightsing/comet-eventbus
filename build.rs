#[cfg(feature = "bridge")]
fn main() {
    tonic_build::configure()
        .out_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bridge"))
        .compile(&["proto/bridge.proto"], &["proto/"])
        .unwrap();
}

#[cfg(not(feature = "bridge"))]
fn main() {}

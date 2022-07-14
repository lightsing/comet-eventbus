#[cfg(feature = "bridge")]
fn main() {
    tonic_build::compile_protos("proto/bridge.proto").unwrap();
}

#[cfg(not(feature = "bridge"))]
fn main() {}

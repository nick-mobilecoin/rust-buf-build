use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "proto";
    println!("cargo:rerun-if-changed={}", proto_root);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("gen");
    protoc_grpcio::compile_grpc_protos(
        &["helloworld.proto"],
        &[proto_root],
        &out_dir,
        None
    ).expect("Failed to compile gRPC definitions!");
    Ok(())
}

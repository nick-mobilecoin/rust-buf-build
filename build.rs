use std::{env, fs};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "proto";
    println!("cargo:rerun-if-changed={}", proto_root);
    // Note: That compile_grpc_protos() expects the directory to exist so one
    // must create subdirectories as needed.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("gen");
    fs::create_dir_all(&out_dir).unwrap();

    // Creating a wrapping mod due to some issues including the files directly,
    // see https://github.com/google/flatbuffers/issues/6261
    fs::write(out_dir.join("mod.rs"), "pub mod helloworld;\npub mod helloworld_grpc;").unwrap();

    protoc_grpcio::compile_grpc_protos(
        &["helloworld.proto"],
        &[proto_root],
        &out_dir,
        None
    ).expect("Failed to compile gRPC definitions!");
    Ok(())
}

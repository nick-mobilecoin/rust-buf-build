use std::process::Command;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // This could also be read in and output as json
    let config_file = manifest_dir.join("buf.gen.yaml");

    let mut file = File::create("env_vars.txt").unwrap();

    for (env, value) in std::env::vars() {
        let line = format!("{env}: {value}\n");
        file.write(line.as_ref()).unwrap();
    }

    let mut cmd = Command::new("buf");
    cmd.arg("generate").arg("buf.build/nick-mc/enclaveapis").arg("--template").arg(config_file);
    // buf treats output as relative to the output directory
    cmd.current_dir(env::var("OUT_DIR").unwrap());
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    let _ = cmd.output().unwrap();
    Ok(())
}

use std::process::Command;
use std::env;
use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dump_env_vars();

    // we could dump this yaml file as json and pass the json string with the
    // `--template` arg, see <https://docs.buf.build/generate/usage#multiple-templates>
    // we could also build up the json here and not use a buf.gen.yaml,
    // this would allow leveraging `OUT_DIR` in the json
    let cwd = current_dir().unwrap();
    let config_file = cwd.join("buf.gen.yaml");

    let mut cmd = Command::new("buf");
    cmd.arg("generate").arg("buf.build/nick-mc/enclaveapis").arg("--template").arg(config_file);

    // buf treats output in the `buf.gen.yaml` as relative to the cwd
    cmd.current_dir(env::var("OUT_DIR").unwrap());
    let _ = cmd.output().unwrap();
    Ok(())
}

/// dirty hacks to see env vars
fn dump_env_vars() {
    let mut file = File::create("env_vars.txt").unwrap();
    for (env, value) in env::vars() {
        let line = format!("{env}: {value}\n");
        file.write(line.as_ref()).unwrap();
    }
}

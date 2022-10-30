use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("provider.proto")?;
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    Command::new("echo")
        .arg(out_dir)
        .spawn()
        .expect("failed to spawn process");
    Ok(())
}
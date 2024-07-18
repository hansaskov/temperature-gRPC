use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("temperature_descriptor.bin"))
        .compile(&["proto/temperature.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/temperature.proto")?;
    Ok(())
}

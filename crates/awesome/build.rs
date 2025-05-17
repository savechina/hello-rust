use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("Compiling proto files to {}", out_dir.display());

    tonic_build::compile_protos("proto/helloworld.proto")?;

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
        // .out_dir("./src")
        .compile_protos(&["proto/store.proto"], &["proto"])?;

    Ok(())
}

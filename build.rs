use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to rerun this build script if the proto files change
    println!("cargo:rerun-if-changed=walletrpc/service.proto");
    println!("cargo:rerun-if-changed=walletrpc/compact_formats.proto");

    // Configure the protobuf code generator
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    
    tonic_prost_build::configure()
        .out_dir(&out_dir)
        .compile_well_known_types(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                "walletrpc/service.proto",
                "walletrpc/compact_formats.proto",
            ],
            &["walletrpc/"], // Include path for imports
        )?;

    Ok(())
}
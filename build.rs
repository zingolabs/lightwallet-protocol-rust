fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "rebuild-proto")]
    {
        use std::path::PathBuf;

        let out_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("src/generated");

        println!("cargo:rerun-if-changed=walletrpc/service.proto");
        println!("cargo:rerun-if-changed=walletrpc/compact_formats.proto");

        tonic_prost_build::configure()
            .out_dir(&out_dir)
            .compile_well_known_types(true)
            .protoc_arg("--experimental_allow_proto3_optional")
            .compile_protos(
                &["walletrpc/service.proto", "walletrpc/compact_formats.proto"],
                &["walletrpc/"],
            )?;
    }

    Ok(())
}

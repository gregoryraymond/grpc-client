use std::path::PathBuf;
use grpc_build::base::prepare_out_dir;
use grpc_build::base::refactor;
use log::debug;
use log::info;

use crate::compile::compile_protogen;

pub(crate) fn grpc_build(protos: Vec<PathBuf>) -> anyhow::Result<Vec<PathBuf>> {
    let mut proto_out_dir = std::env::current_dir()?;
    proto_out_dir.push("protogen");

    let mut target_dir = proto_out_dir.clone();
    target_dir.push("target");

    let location = String::from(proto_out_dir.to_string_lossy()) + "/Cargo.toml";

    if target_dir.exists() {
        debug!("Creating proto {}", proto_out_dir.to_string_lossy());
        let _ = std::fs::remove_dir_all(&proto_out_dir);
        let _ = std::fs::create_dir(&proto_out_dir)?;

        prepare_out_dir(&proto_out_dir)?;

        let mut path = protos[0].clone();
        path.pop();

        // Input is shadowed in the proto-path by 
        tonic_build::configure()
            .out_dir(&proto_out_dir)
            .build_server(true)
            .build_client(true)
            .compile_well_known_types(true)
            .compile(&protos, &[path.to_string_lossy().to_string().as_str(), "."])?;

        refactor(&proto_out_dir)?;

        info!("Writing cargo toml {}", location);
        std::fs::write(&location, r#"
        [package]
        name = "protogen"
        version = "0.1.0"
        edition = "2021"
        
        [dependencies]
        grpc-build = "6.1.0"
        tonic-build = "0.11"
        tonic = "*"
        prost = "*"
        
        [build-dependencies]
        syn = { version = "2.0.65", features = ["full"]}
        walkdir = "2.5.0"
        
        [lib]
        crate-type = ["cdylib"]
        bench = false
        path = "mod.rs"        
"#)?;
    }
    
    let compilation = compile_protogen(PathBuf::from(&location))?;
    Ok(compilation)
}
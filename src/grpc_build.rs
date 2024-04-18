use std::path::PathBuf;
use std::str::FromStr;
use druid::FileInfo;
use grpc_build::base::prepare_out_dir;
use grpc_build::base::refactor;

use crate::compile::compile_protogen;

pub(crate) fn grpc_build(protos: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let proto_out_dir = "protogen";

    prepare_out_dir(proto_out_dir)?;

    let mut path = protos[0].clone();
    path.pop();

    tonic_build::configure()
        .out_dir(proto_out_dir)
        .build_server(true)
        .build_client(true)
        .compile(&protos, &[".", path.to_string_lossy().to_string().as_str()])?;

    refactor(proto_out_dir)?;

    let mut location = PathBuf::from_str(proto_out_dir)?;
    location.set_file_name("mod.rs");
    compile_protogen(location)?;

    Ok(())
}
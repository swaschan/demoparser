use std::{env, io::Result, path::PathBuf};

const PROTO_FILES: &[&str] = &[
    "steammessages.proto",
    "gcsdk_gcmessages.proto",
    "demo.proto",
    "cstrike15_gcmessages.proto",
    "cstrike15_usermessages.proto",
    "usermessages.proto",
    "networkbasetypes.proto",
    "engine_gcmessages.proto",
    "netmessages.proto",
    "network_connection.proto",
    "cs_usercmd.proto",
    "usercmd.proto",
    "gameevents.proto",
    "cs_gameevents.proto",
];

fn main() -> Result<()> {
    println!("cargo::rerun-if-env-changed=CSGOPROTO_REGENERATE");
    println!("cargo::rerun-if-env-changed=CSGOPROTO_PROTO_DIR");

    if env::var_os("CSGOPROTO_REGENERATE").is_none() {
        return Ok(());
    }

    let proto_dir = PathBuf::from(env::var_os("CSGOPROTO_PROTO_DIR").expect("CSGOPROTO_PROTO_DIR must be set"));
    let protos: Vec<_> = PROTO_FILES.iter().map(|file| proto_dir.join(file)).collect();

    prost_build::Config::new()
        .format(false)
        .out_dir("src")
        .default_package_filename("protobuf")
        .bytes(["."])
        .enum_attribute(".", "#[derive(::strum::EnumIter)]")
        .compile_protos(&protos, &[proto_dir])
}

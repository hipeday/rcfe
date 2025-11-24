fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "proto/rpc.proto",
                "proto/kv.proto",
                "proto/auth.proto",
                "proto/election.proto",
                "proto/lock.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}

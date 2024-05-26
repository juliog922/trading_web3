/// Compile Protocol Buffers files into Rust files using Tonic.
///
/// This function configures the compilation process for Protocol Buffers files
/// and compiles them into Rust files using Tonic.
fn main () {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/auth")
        .compile(
            &["./common/proto/auth_service.proto"],
            &["./common/proto/"] 
        )
        .unwrap();
}
fn main () {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/auth")
        .compile(
            &["../common/proto/auth_service.proto"],
            &["../common/proto/"] 
        )
        .unwrap();
}
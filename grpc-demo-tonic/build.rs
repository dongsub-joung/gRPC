fn main(){
    let proto_file= "./proto/hello.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| {
            panic!("protobuf compile error: {}", e);
        });

        println!("cargo:return-if-changed= {}", proto_file);

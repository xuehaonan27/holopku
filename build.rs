//! Build script for service codegen.

fn main() {
    tonic_build::configure()
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/post.proto"], &["proto/api/v1"])
        .unwrap();

    tonic_build::configure()
        .protoc_arg("--proto_path=.")
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/sellPost.proto"], &["proto/api/v1"])
        .unwrap();

    tonic_build::configure()
        .protoc_arg("--proto_path=.")
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/foodPost.proto"], &["proto/api/v1"])
        .unwrap();

    tonic_build::configure()
        .protoc_arg("--proto_path=.")
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/amusementPost.proto"], &["proto/api/v1"])
        .unwrap();

    tonic_build::configure()
        .protoc_arg("--proto_path=.")
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/forum.proto"], &["proto/api/v1"])
        .unwrap();

    tonic_build::configure()
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/auth.proto"], &["proto/api/v1"])
        .unwrap();
}

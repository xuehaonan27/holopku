//! Build script for service codegen.

fn main() {
    tonic_build::configure()
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/forum.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .out_dir("src/codegen")
        .compile(&["proto/api/v1/auth.proto"], &["proto"])
        .unwrap();
}

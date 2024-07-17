fn main() {
    prost_build::compile_protos(
        &["../protos/accounts/login/email/request.proto"],
        &["../protos/accounts/login/email/"])
        .unwrap();
}

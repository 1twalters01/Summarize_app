use std::{fs, io::ErrorKind, path::{Path, PathBuf}};

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set");

    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    let proto_files = &[
        "../protos/accounts/login/email/request.proto",
        "../protos/accounts/login/email/response.proto",
    ];
    let proto_include_dirs = &["../protos/accounts/login/email"];
    prost_config.compile_protos(proto_files, proto_include_dirs).unwrap();

    let mut custom_file_path = "accounts/login/email_request.rs";
    let mut generated_file: PathBuf = Path::new(&out_dir).join("request.rs");
    create_dirs(&out_dir, &custom_file_path.split("/").collect::<Vec<&str>>());
    rename_file(&out_dir, generated_file, custom_file_path);

    custom_file_path = "accounts/login/email_response.rs";
    generated_file = Path::new(&out_dir).join("response.rs");
    rename_file(&out_dir, generated_file, custom_file_path);
}

fn create_dirs(path_root: &str, custom_vec: &Vec<&str>) {
    match fs::remove_dir_all(Path::new(path_root).join(custom_vec[0])) {
        Ok(_) => {},
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {},
                _ => panic!("failed to delete folder")
            }
        }
    }
    let mut path: String = String::new();
    for folder in custom_vec[..custom_vec.len() - 1].iter() {
        path = path + folder;
        fs::create_dir(Path::new(path_root).join(path.to_string()))
            .expect("Unable to create directory");
        path = path + "/";
    }
}

fn rename_file(dir: &str, generated_file: PathBuf, custom_file_path: &str) {
    let custom_file: PathBuf = Path::new(dir).join(custom_file_path);
    fs::rename(generated_file, custom_file).expect("failed to rename generated file");
}

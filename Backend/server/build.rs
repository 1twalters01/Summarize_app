use std::{fs, io::ErrorKind, path::{Path, PathBuf}};

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set");

    let proto_files = &["../protos/accounts/login/email/request.proto"];
    let proto_include_dirs = &["../protos/accounts/login/email"];
    prost_build::compile_protos(proto_files, proto_include_dirs).unwrap();

    let custom_file_path = "accounts/login/email_request.rs";
    let generated_file: PathBuf = Path::new(&out_dir).join("request.rs");
    rename_file(out_dir, generated_file, custom_file_path)
}

fn create_dirs(path_root: &str, custom_vec: &Vec<&str>) {
    let mut path: String = String::new();
    for folder in custom_vec[..custom_vec.len() - 1].iter() {
        path = path + folder;
        fs::create_dir(Path::new(path_root).join(path.to_string())).expect(&path);
        path = path + "/";
    }
}

fn rename_file(dir: String, generated_file: PathBuf, custom_file_path: &str) {
    let custom_vec = custom_file_path.split("/").collect::<Vec<&str>>();
    let custom_file: PathBuf = Path::new(&dir).join(custom_file_path);
    match fs::remove_dir_all(Path::new(&dir).join(custom_vec[0])) {
        Ok(_) => {},
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {},
                _ => panic!("failed to delete folder")
            }
        }
    }
    create_dirs(&dir, &custom_vec);
    fs::rename(generated_file, custom_file).expect("failed to rename generated file");
}

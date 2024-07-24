use std::{fs, io::{self, ErrorKind}, path::{Path, PathBuf}};

fn main() {
    let out_dir = "src/generated/protos";
    delete_files_in_dir(out_dir).expect("error cleaning output directory");

    let mut prost_config = prost_build::Config::new();
    prost_config.out_dir(out_dir);

    // login email protobufs 
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    let proto_files = &[
        "../protos/accounts/login/email/request.proto",
        "../protos/accounts/login/email/response.proto",
    ];
    let proto_include_dirs = &["../protos/accounts/login/email"];
    prost_config.compile_protos(proto_files, proto_include_dirs).expect("Failed to compile protos");

    let mut custom_file_path = "accounts/login/email/request.rs";
    let mut generated_file: PathBuf = Path::new(&out_dir).join("accounts.login.email.request.rs");
    create_dirs(&out_dir, &custom_file_path.split("/").collect::<Vec<&str>>());
    rename_file(&out_dir, generated_file, custom_file_path);

    custom_file_path = "accounts/login/email/response.rs";
    generated_file = Path::new(&out_dir).join("accounts.login.email.response.rs");
    rename_file(&out_dir, generated_file, custom_file_path);
    // panic!("email build done");


    // login password protobufs
    let proto_files = &[
        "../protos/accounts/login/password/request.proto",
        "../protos/accounts/login/password/response.proto",
        "../protos/accounts/auth_tokens.proto",
    ];
    let proto_include_dirs = &["../protos"];
    prost_config.compile_protos(proto_files, proto_include_dirs).unwrap();
    
    custom_file_path = "accounts/login/password/request.rs";
    generated_file = Path::new(&out_dir).join("accounts.login.password.request.rs");
    create_dirs(&out_dir, &custom_file_path.split("/").collect::<Vec<&str>>());
    rename_file(&out_dir, generated_file, custom_file_path);
    // panic!("password request made");

    custom_file_path = "accounts/login/password/response.rs";
    generated_file = Path::new(&out_dir).join("accounts.login.password.response.rs");
    rename_file(&out_dir, generated_file, custom_file_path);

    custom_file_path = "accounts/auth_tokens.rs";
    generated_file = Path::new(&out_dir).join("accounts.auth_tokens.rs");
    rename_file(&out_dir, generated_file, custom_file_path);
}

fn delete_files_in_dir<P: AsRef<Path>>(root_dir: P) -> io::Result<()> {
    fn visit_dirs(dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    visit_dirs(&path).unwrap();
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "rs" && path.file_name().unwrap() != "mod.rs" {
                            fs::remove_file(&path).unwrap();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    visit_dirs(root_dir.as_ref())
}

fn create_dirs(path_root: &str, custom_vec: &Vec<&str>) {
    let mut path: String = String::new();
    for folder in custom_vec[..custom_vec.len() - 1].iter() {
        path = path + folder;
        match fs::create_dir(Path::new(path_root).join(path.to_string())){
            Ok(_) => {},
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {},
            _ => panic!("unable to create file"),
        };
        path = path + "/";
    }
}

fn rename_file(dir: &str, generated_file: PathBuf, custom_file_path: &str) {
    let custom_file: PathBuf = Path::new(dir).join(custom_file_path);
    fs::rename(generated_file, custom_file).expect("failed to rename generated file");
}

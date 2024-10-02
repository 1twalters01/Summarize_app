use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = "src/generated/protos";
    let base_dir = "../../protos";
    delete_rust_files_in_dir(out_dir).expect("error cleaning output directory");

    let mut prost_config = prost_build::Config::new();
    prost_config.out_dir(out_dir);
    let protobuf_filename_vec: &Vec<&str> = &Vec::from([
        "../../protos/accounts/auth_tokens.proto",
        "../../protos/accounts/login/email/request.proto",
        "../../protos/accounts/login/email/response.proto",
        "../../protos/accounts/login/password/request.proto",
        "../../protos/accounts/login/password/response.proto",
        "../../protos/accounts/login/totp/request.proto",
        "../../protos/accounts/login/totp/response.proto",
        "../../protos/accounts/register/email/request.proto",
        "../../protos/accounts/register/email/response.proto",
        "../../protos/accounts/register/verification/request.proto",
        "../../protos/accounts/register/verification/response.proto",
        "../../protos/accounts/register/details/request.proto",
        "../../protos/accounts/register/details/response.proto",
        "../../protos/accounts/password_reset/email/request.proto",
        "../../protos/accounts/password_reset/email/response.proto",
        "../../protos/accounts/password_reset/verification/request.proto",
        "../../protos/accounts/password_reset/verification/response.proto",
        "../../protos/accounts/password_reset/password/request.proto",
        "../../protos/accounts/password_reset/password/response.proto",
    ]);
    generate_files_from_protobufs(prost_config, out_dir, protobuf_filename_vec, base_dir);

    prost_config = prost_build::Config::new();
    prost_config.out_dir(out_dir);
    let protobuf_filename_vec: &Vec<&str> = &Vec::from([
        "../../protos/settings/profile/confirmation.proto",
        "../../protos/settings/profile/email/request.proto",
        "../../protos/settings/profile/email/response.proto",
        "../../protos/settings/profile/name/request.proto",
        "../../protos/settings/profile/name/response.proto",
        "../../protos/settings/profile/password/request.proto",
        "../../protos/settings/profile/password/response.proto",
        "../../protos/settings/profile/username/request.proto",
        "../../protos/settings/profile/username/response.proto",
        "../../protos/settings/profile/language/request.proto",
        "../../protos/settings/profile/language/response.proto",
        "../../protos/settings/profile/theme/request.proto",
        "../../protos/settings/profile/theme/response.proto",
        "../../protos/settings/profile/totp/request.proto",
        "../../protos/settings/profile/totp/response.proto",
    ]);
    generate_files_from_protobufs(prost_config, out_dir, protobuf_filename_vec, base_dir);
}

fn generate_files_from_protobufs(
    mut prost_config: prost_build::Config,
    out_dir: &str,
    protobuf_filename_vec: &Vec<&str>,
    protobuf_base_dir: &str,
) {
    let proto_include_dirs: &[&str] = &[protobuf_base_dir];
    let proto_files: &[&str] = protobuf_filename_vec;
    prost_config
        .compile_protos(proto_files, proto_include_dirs)
        .expect("Failed to compile protos");

    let file_paths: Vec<String> = proto_files
        .into_iter()
        .map(|file_path| {
            file_path
                .strip_prefix(&format!("{}{}", protobuf_base_dir, "/"))
                .unwrap()
                .replace(".proto", ".rs")
        })
        .collect();

    let generated_file_paths: Vec<PathBuf> = file_paths
        .iter()
        .map(|file_path| {
            Path::new(out_dir).join(file_path.replace("/", ".").replace(".proto", ".rs"))
        })
        .collect();

    for i in 0..file_paths.len() {
        rename_file(&out_dir, generated_file_paths[i].clone(), &file_paths[i]);
    }
}

fn delete_rust_files_in_dir<P: AsRef<Path>>(root_dir: P) -> io::Result<()> {
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
        match fs::create_dir(Path::new(path_root).join(path.to_string())) {
            Ok(_) => {},
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {},
            _ => panic!("unable to create file"),
        };
        path = path + "/";
    }
}

fn rename_file(dir: &str, generated_file: PathBuf, custom_file_path: &str) {
    create_dirs(dir, &custom_file_path.split("/").collect::<Vec<&str>>());
    let custom_file: PathBuf = Path::new(dir).join(custom_file_path);
    fs::rename(generated_file, custom_file).expect("failed to rename generated file");
}

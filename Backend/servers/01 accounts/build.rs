use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = "src/generated/protos";
    let base_dir = "../../../Interface/protos";
    delete_rust_files_in_dir(out_dir).expect("error cleaning output directory");

    let mut prost_config = prost_build::Config::new();
    prost_config.out_dir(out_dir);
    let protobuf_location_vec: &Vec<&str> = &Vec::from([
        "/accounts/auth_tokens.proto",
        "/accounts/login/email/request.proto",
        "/accounts/login/email/response.proto",
        "/accounts/login/password/request.proto",
        "/accounts/login/password/response.proto",
        "/accounts/login/totp/request.proto",
        "/accounts/login/totp/response.proto",
        "/accounts/login/refresh/response.proto",
        "/accounts/register/email/request.proto",
        "/accounts/register/email/response.proto",
        "/accounts/register/verification/request.proto",
        "/accounts/register/verification/response.proto",
        "/accounts/register/details/request.proto",
        "/accounts/register/details/response.proto",
        "/accounts/password_reset/email/request.proto",
        "/accounts/password_reset/email/response.proto",
        "/accounts/password_reset/verification/request.proto",
        "/accounts/password_reset/verification/response.proto",
        "/accounts/password_reset/password/request.proto",
        "/accounts/password_reset/password/response.proto",
        "/accounts/captcha/get/response.proto",
        "/accounts/captcha/verification/request.proto",
        "/accounts/captcha/verification/response.proto",
    ]);
    let filename_vec = &protobuf_location_vec
        .into_iter()
        .map(|location| base_dir.to_string() + location)
        .collect::<Vec<String>>();
    generate_files_from_protobufs(prost_config, out_dir, filename_vec, base_dir);

    prost_config = prost_build::Config::new();
    prost_config.out_dir(out_dir);
    let protobuf_location_vec: &Vec<&str> = &Vec::from([
        "/settings/profile/confirmation/request.proto",
        "/settings/profile/confirmation/response.proto",
        "/settings/profile/email/request.proto",
        "/settings/profile/email/response.proto",
        "/settings/profile/name/request.proto",
        "/settings/profile/name/response.proto",
        "/settings/profile/password/request.proto",
        "/settings/profile/password/response.proto",
        "/settings/profile/username/request.proto",
        "/settings/profile/username/response.proto",
        "/settings/profile/language/request.proto",
        "/settings/profile/language/response.proto",
        "/settings/profile/theme/request.proto",
        "/settings/profile/theme/response.proto",
        "/settings/profile/totp/request.proto",
        "/settings/profile/totp/response.proto",
    ]);
    let filename_vec = &protobuf_location_vec
        .into_iter()
        .map(|location| base_dir.to_string() + location)
        .collect::<Vec<String>>();
    generate_files_from_protobufs(prost_config, out_dir, filename_vec, base_dir);
}

fn generate_files_from_protobufs(
    mut prost_config: prost_build::Config,
    out_dir: &str,
    filename_vec: &Vec<String>,
    base_dir: &str,
) {
    let proto_include_dirs: &[&str] = &[base_dir];
    let proto_files: &[String] = filename_vec;
    prost_config
        .compile_protos(proto_files, proto_include_dirs)
        .expect("Failed to compile protos");

    let file_paths: Vec<String> = proto_files
        .into_iter()
        .map(|file_path| {
            file_path
                .strip_prefix(&format!("{}{}", base_dir, "/"))
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
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
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

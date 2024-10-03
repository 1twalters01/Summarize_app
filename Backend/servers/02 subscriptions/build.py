import os, subprocess, sys, pathlib

def main():
    if __name__ == "build":
        out_dir = "src/generated/protos"
        base_dir = "../../../Interface/protos"
        delete_python_files_in_dir(out_dir)

        protobuf_locations = [
            "/subscriptions/status.proto",
            # "/subscriptions/stripe.proto",
            # "/subscriptions/paypal.proto",
            # "/settings/subscription",
        ]
        protobuf_filenames = [base_dir + location for location in protobuf_locations]
        generate_files_from_protobufs(out_dir, protobuf_filenames, base_dir)

def generate_files_from_protobufs(out_dir, protobuf_filenames, base_dir):
    file_paths = [filename.removeprefix(f"{base_dir}/").replace(".proto", "") for filename in protobuf_filenames]
    output_filenames = [f"{out_dir}/{file_path}_pb2.py" for file_path in file_paths]
    new_filenames = [f"{out_dir}/{file_path}.py" for file_path in file_paths]
    print(file_paths)
    print(output_filenames)
    print(new_filenames)

    commands = [f"protoc \
        --proto_path={base_dir} \
        --python_out={out_dir} \
        --experimental_allow_proto3_optional {filename}" for filename in protobuf_filenames]
    run_commands(commands)

    for index, output_filename in enumerate(output_filenames):
        os.rename(f"{output_filename}", f"{new_filenames[index]}")
    print(commands)

def run_commands(commands):
    for command in commands:
        try:
            subprocess.run(command, shell=True, check=True)
        except subprocess.CalledProcessError as e:
            print(f"Command failed with return code {e.returncode}")
            print(f"Error message: {e}")
            sys.exit()

def delete_python_files_in_dir(dir):
    if os.path.isdir(dir):
        for dirpath, _, filenames in os.walk(dir):
            for filename in filenames:
                file_path = pathlib.Path(dirpath) / filename
                if file_path.suffix == ".py" and file_path.stem != "__init__":
                    os.remove(file_path)

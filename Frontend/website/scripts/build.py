import os
from command_functions import run_command, run_commands

def compile_protobuffers_for(file_pairs):
    success_text = "protobuf compiled succesfully"
    for file_pair in file_pairs:
        output_file_name = file_pair["out"]
        root = file_pair["in"]["root"]
        file_path = file_pair["in"]["path"]
        input_file = root + file_path
        try:
            dependencies = ' '.join(file_pair["deps"]) + ' '
        except:
            dependencies = ""

        path = "src/protos/"
        proto_path = ""
        output_list = output_file_name.split("/")[3:]
        for folder in output_list:
            proto_path += folder + "/"
            path += folder

            if os.path.exists(path) == False:
                run_command(f"mkdir {path}")

            path += "/"

        if dependencies == "":
            js_command_pair = {
                "command": f"protoc \
                        --proto_path={backend_protobuf_location} \
                        --js_out=import_style=commonjs,binary:{output_file_name} \
                        --experimental_allow_proto3_optional {input_file}",
                "success_text": success_text
            }
            ts_command_pair = {
                "command": f"protoc \
                        --proto_path={backend_protobuf_location} \
                        --plugin=protoc-gen-ts=$(which protoc-gen-ts) \
                        --ts_out={output_file_name} \
                        --experimental_allow_proto3_optional {input_file}",
                "success_text": success_text
            }
        else:
            js_command_pair = {
                    "command": f"protoc \
                            --proto_path={backend_protobuf_location} \
                            --js_out=import_style=commonjs,binary:{output_file_name} \
                            --experimental_allow_proto3_optional {dependencies} {input_file}",
                "success_text": success_text
            }
            ts_command_pair = {
                "command": f"protoc \
                        --proto_path={backend_protobuf_location} \
                        --plugin=protoc-gen-ts=$(which protoc-gen-ts) \
                        --ts_out={output_file_name} \
                        --experimental_allow_proto3_optional {dependencies} {input_file}",
                "success_text": success_text
            }

        # run_commands([js_command_pair, ts_command_pair])
        run_commands([ts_command_pair])

        print("run complete")

def webpack_build():
    command = "npx webpack build"
    run_command(command, "")


backend_protobuf_location = "../../Interface/protos"
frontend_protobuf_location = "./src/protos"

file_pair_1 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/auth_tokens.proto"
    }
}
file_pair_2 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/email/request.proto"
    }
}
file_pair_3 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/email/response.proto"
    }
}
file_pair_4 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/password/request.proto"
    }
}
file_pair_5 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/password/response.proto",
    },
    "deps": [f"{backend_protobuf_location}/accounts/auth_tokens.proto",]
}
file_pair_6 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/totp/request.proto"
    }
}
file_pair_7 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/login/totp/response.proto",
    },
    "deps": [f"{backend_protobuf_location}/accounts/auth_tokens.proto",]
}
file_pair_8 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/email/request.proto"
    }
}
file_pair_9 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/email/response.proto"
    }
}
file_pair_10 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/verification/request.proto"
    }
}
file_pair_11 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/verification/response.proto"
    }
}
file_pair_12 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/details/request.proto"
    }
}
file_pair_13 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/register/details/response.proto"
    }
}
file_pair_14 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/email/request.proto"
    }
}
file_pair_15 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/email/response.proto"
    }
}
file_pair_16 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/verification/request.proto"
    }
}
file_pair_17 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/verification/response.proto"
    }
}
file_pair_18 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/password/request.proto"
    }
}
file_pair_19 = {
    "out": frontend_protobuf_location,
    "in": { 
        "root": backend_protobuf_location,
        "path": "/accounts/password_reset/password/response.proto"
    }
}

run_command(f"rm -r {frontend_protobuf_location}")
run_command(f"mkdir {frontend_protobuf_location}")
compile_protobuffers_for([file_pair_1])
compile_protobuffers_for([file_pair_2])
compile_protobuffers_for([file_pair_3])
compile_protobuffers_for([file_pair_4])
compile_protobuffers_for([file_pair_5])
compile_protobuffers_for([file_pair_6])
compile_protobuffers_for([file_pair_7])
compile_protobuffers_for([file_pair_8])
compile_protobuffers_for([file_pair_9])
compile_protobuffers_for([file_pair_10])
compile_protobuffers_for([file_pair_11])
compile_protobuffers_for([file_pair_12])
compile_protobuffers_for([file_pair_13])
compile_protobuffers_for([file_pair_14])
compile_protobuffers_for([file_pair_15])
compile_protobuffers_for([file_pair_16])
compile_protobuffers_for([file_pair_17])
compile_protobuffers_for([file_pair_18])
compile_protobuffers_for([file_pair_19])
run_command("npx webpack build")

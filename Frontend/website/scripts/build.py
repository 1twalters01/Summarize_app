import os
from command_functions import run_command, run_commands

def compile_protobuffers_for(file_pairs):
    success_text = "protobuf compiled succesfully"
    for file_pair in file_pairs:
        output_file_name = file_pair["out"]
        input_file = file_pair["in"]
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

        print(proto_path)
        if dependencies == "":
            print(f"input_file: {input_file}")
            print(f"output_file: {output_file_name}")
            js_command_pair = {
                "command": f"protoc --proto_path={backend_protobuf_location} --js_out=import_style=commonjs,binary:{output_file_name} --experimental_allow_proto3_optional {input_file}",
                "success_text": success_text
            }
        else:
            js_command_pair = {
                    "command": f"protoc --proto_path={backend_protobuf_location} --js_out=import_style=commonjs,binary:{output_file_name} --experimental_allow_proto3_optional {dependencies} {input_file}",
                "success_text": success_text
            }

        # run_commands([js_command_pair, ts_command_pair])
        run_commands([js_command_pair])
        print("run complete")

def webpack_build():
    command = "npx webpack build"
    run_command(command, "")


backend_protobuf_location = "../../Backend/protos"
frontend_protobuf_location = "./src/protos"

# file_pair_1 = {
#     "out": f"{frontend_protobuf_location}/accounts/login/email/request",
#     "in": f"{backend_protobuf_location}/accounts/login/email/request.proto"
# }
# file_pair_2 = {
#     "out": f"{frontend_protobuf_location}/accounts/login/email/response",
#     "in": f"{backend_protobuf_location}/accounts/login/email/response.proto"
# }
# file_pair_3 = {
#     "out": f"{frontend_protobuf_location}/accounts/login/password/request",
#     "in": f"{backend_protobuf_location}/accounts/login/password/request.proto"
# }
# file_pair_4 = {
#     "out": f"{frontend_protobuf_location}/accounts/login/password/response",
#     "in": f"{backend_protobuf_location}/accounts/login/password/response.proto",
#     "deps": [
#         f"{backend_protobuf_location}/accounts/auth_tokens.proto",
#     ]
# }
# file_pair_5 = {
#     "out": f"{frontend_protobuf_location}/accounts/auth_tokens",
#     "in": f"{backend_protobuf_location}/accounts/auth_tokens.proto"
# }

file_pair_1 = {
    "out": f"{frontend_protobuf_location}",
    "in": f"{backend_protobuf_location}/accounts/login/email/request.proto"
}
file_pair_2 = {
    "out": f"{frontend_protobuf_location}",
    "in": f"{backend_protobuf_location}/accounts/login/email/response.proto"
}
file_pair_3 = {
    "out": f"{frontend_protobuf_location}",
    "in": f"{backend_protobuf_location}/accounts/login/password/request.proto"
}
file_pair_4 = {
    "out": f"{frontend_protobuf_location}",
    "in": f"{backend_protobuf_location}/accounts/login/password/response.proto",
    "deps": [
        f"{backend_protobuf_location}/accounts/auth_tokens.proto",
    ]
}

run_command(f"rm -r {frontend_protobuf_location}")
run_command(f"mkdir {frontend_protobuf_location}")
compile_protobuffers_for([file_pair_1])
compile_protobuffers_for([file_pair_2])
compile_protobuffers_for([file_pair_3])
compile_protobuffers_for([file_pair_4])
# compile_protobuffers_for([file_pair_5])
run_command("npx webpack build")

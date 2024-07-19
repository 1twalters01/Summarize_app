import os
from command_functions import run_command, run_commands

def compile_protobuffers_for(file_pairs):
    success_text = "protobuf compiled succesfully"
    for file_pair in file_pairs:
        output_file_name = file_pair["out"]
        input_file = file_pair["in"]

        path = "src/protos/"
        output_list = output_file_name.split("/")[2:-1]
        for folder in output_list:
            path += folder

            if os.path.exists(path) == False:
                run_command(f"mkdir {path}")

            path += "/"

        js_command_pair = {
            "command": f"npx pbjs --es6 {output_file_name}.js {input_file}",
            "success_text": success_text
        }
        ts_command_pair = {
            "command": f"npx pbjs --ts {output_file_name}.ts {input_file}",
            "success_text": success_text
        }
        run_commands([js_command_pair, ts_command_pair])

def webpack_build():
    command = "npx webpack build"
    run_command(command, "")


backend_protobuf_location = "../../../Backend/protos"
frontend_protobuf_location = "src/protos"

file_pair_1 = {
    "out": f"{frontend_protobuf_location}/accounts/login/email_request",
    "in": f"{backend_protobuf_location}/accounts/login/email/request.proto"
}
file_pair_2 = {
    "out": f"{frontend_protobuf_location}/accounts/login/email_response",
    "in": f"{backend_protobuf_location}/accounts/login/email/response.proto"
}

run_command(f"rm -r {frontend_protobuf_location}")
run_command(f"mkdir {frontend_protobuf_location}")
compile_protobuffers_for([file_pair_1])
compile_protobuffers_for([file_pair_2])
run_command("npx webpack build")

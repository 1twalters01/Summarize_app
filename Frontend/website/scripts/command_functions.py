import subprocess, sys

def run_command(command, success_text=""):
    try:
        subprocess.run(command, shell=True, check=True)
        if success_text == "":
            print(success_text, end="")
        else:
            print(success_text)

    except subprocess.CalledProcessError as e:
        print(f"Command failed with return code {e.returncode}")
        print(f"Error message: {e}")
        sys.exit()


def run_commands(command_dicts):
    for command_dict in command_dicts:
        command = command_dict["command"]
        try:
            success_text = command_dict["success_text"]
        except:
            success_text = ""
        run_command(command, success_text)


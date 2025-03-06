import subprocess

def run_command(command, arguments):
    
    command_options = ""
    for entry in arguments:
        command_options+=(entry+ " ")
    
    shell_command =  command + " " +command_options
    subprocess.Popen(shell_command, shell=True)
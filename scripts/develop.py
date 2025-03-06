import sys 
from utils.platform import parse_platform
from utils.shell import run_command

platform = parse_platform(sys.argv[1]);
if platform == "android":
    run_command(  "cargo", [ "tauri", "android", "dev"])
elif platform =="ios":
    run_command(  "cargo", [ "tauri", "ios", "dev"])
else:
    print("Unsupported platform")
    exit(1)
    
    
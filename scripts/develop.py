import sys 
from utils.platform import parse_platform
from utils.shell import run_command

platform = parse_platform(sys.argv[1]);

run_command("echo", ["a", "b", "c"])
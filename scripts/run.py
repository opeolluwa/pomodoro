from utils.platform import parse_service_platform
from utils.shell import run_command
import sys

platform = parse_service_platform(sys.argv[1])

if platform == "backend":
    run_command("docker", ["compose", "-f ", "'./backend/docker-compose.yaml'", "up", "-d"])
else:
    print(f"Invalid platform: {platform} was supplied")
    exit(1)

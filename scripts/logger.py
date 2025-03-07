from utils.platform import parse_service_platform
from utils.shell import run_command
import sys

platform = parse_service_platform(sys.argv[1])


if platform == "backend":
    run_command(
        "docker",
        ["compose", "logs", "-f", "'./backend/docker-compose.yaml'", "poseidon"],
    )

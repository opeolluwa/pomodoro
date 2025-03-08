
# from utils.logger import log_error

def parse_app_platform(platform):
    platform = str(platform).lower().strip()
    if platform == "android" or platform == "and" or platform == "a":
        return "android"
    elif platform == "ios" or platform == "i":
        return "ios"
    else:
        print("Invalid platform:\nUse on of \"android\" or\"ios\"")
        exit(1)
        

def parse_service_platform(platform):
    platform = str(platform).lower().strip()
    if platform  == 'backend' or platform == "b":
        return "backend"
    else:
        print(f"unsupported platform {platform}")
        exit(1)
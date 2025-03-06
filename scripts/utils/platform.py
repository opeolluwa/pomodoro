
# from utils.logger import log_error

def parse_platform(platform):
    platform = str(platform)
    if platform == "android" or platform == "and" or platform == "a":
        return "android"
    elif platform == "ios" or platform == "i":
        return "ios"
    else:
        print("Invalid platform:\nUse on of \"android\" or\"ios\"")
        exit(1)
        

import platform

plt = platform.system().lower()

if plt == "windows":
    print("Your system is Windows")
elif plt == "linux":
    print("Your system is Linux")
else:
    print("Unidentified system")

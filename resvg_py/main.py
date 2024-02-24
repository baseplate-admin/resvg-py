import subprocess
from .downloader import download_windows, download_linux
import platform

from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent

plt = platform.system().lower()

if plt == "windows":
    download_windows()
    binary = Path(BASE_DIR, "resvg.exe")
elif plt == "linux":
    download_linux()
    binary = Path(BASE_DIR, "resvg")
else:
    print("Unidentified system")


def main_function(input: str, output: str):
    subprocess.Popen(
        f"{binary} {input} {output}",
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        bufsize=2**12,
    )

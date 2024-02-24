import subprocess
from .downloader import download_windows, download_linux
import platform
import tempfile
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


def main(input: str):
    contents = b""
    with tempfile.NamedTemporaryFile(delete_on_close=True) as f:
        subprocess.Popen(
            [binary, input, f.name],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        f.seek(0)
        contents = f.read()

        f.close()

    return contents

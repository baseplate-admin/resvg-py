import urllib.request
import zipfile
from pathlib import Path
import tempfile

BASE_DIR = Path(__file__).resolve().parent

version = "0.40.0"


def download_windows():
    for file in BASE_DIR.glob("*.exe"):
        if "resvg.exe" in file.name:
            return

    f = urllib.request.urlopen(
        f"https://github.com/RazrFalcon/resvg/releases/download/v{version}/resvg-win64.zip",
    )

    with tempfile.NamedTemporaryFile() as fp:
        fp.write(f.read())

        with zipfile.ZipFile(fp, "r") as zip_ref:
            zip_ref.extractall(path=BASE_DIR)

        fp.close()

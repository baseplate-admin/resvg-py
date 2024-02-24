import urllib.request
import zipfile

from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent

version = "0.40.0"


def download_windows():
    location = Path(BASE_DIR, "win.zip")

    urllib.request.urlretrieve(
        f"https://github.com/RazrFalcon/resvg/releases/download/v{version}/resvg-win64.zip",
        location,
    )

    with zipfile.ZipFile(location, "r") as zip_ref:
        zip_ref.extractall()
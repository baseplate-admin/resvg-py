import urllib.request
import zipfile
from pathlib import Path
import tempfile
import tarfile

BASE_DIR = Path(__file__).resolve().parent

version = "0.40.0"


def _skip_if_file_exists():
    for file in BASE_DIR.glob("*"):
        if "resvg" in file.name:
            return


def download_windows():
    _skip_if_file_exists()

    f = urllib.request.urlopen(
        f"https://github.com/RazrFalcon/resvg/releases/download/v{version}/resvg-win64.zip",
    )

    with tempfile.NamedTemporaryFile() as fp:
        fp.write(f.read())

        with zipfile.ZipFile(fp, "r") as zip_ref:
            zip_ref.extractall(path=BASE_DIR)

        fp.close()


def download_linux():
    _skip_if_file_exists()

    f = urllib.request.urlopen(
        f"https://github.com/RazrFalcon/resvg/releases/download/v{version}/resvg-linux-x86_64.tar.gz",
    )

    with tempfile.NamedTemporaryFile(
        "wb", suffix=".tar.gz", delete_on_close=False
    ) as fp:
        fp.write(f.read())

        with tarfile.open(fp.name, "r:gz") as tar:
            tar.extractall(path=BASE_DIR)

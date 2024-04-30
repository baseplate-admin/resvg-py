import resvg_py
from pathlib import Path
import os

BASE_DIR = Path(__file__).resolve().parent

svg_output = "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"


def test_path():
    path = os.path.join(BASE_DIR, "acid.svg")
    base = resvg_py.svg_to_base64(path)
    print(path)
    assert base == svg_output


def test_gzip_path():
    path = os.path.join(BASE_DIR, "acid.svg.gz")
    base = resvg_py.svg_to_base64(path)
    print(path)

    assert base == svg_output

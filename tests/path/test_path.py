import base64
import json
import os

from pathlib import Path

import resvg_py

BASE_DIR = Path(__file__).resolve().parent

with open(BASE_DIR / "expected_outputs.json", encoding="utf-8") as f:
    _expected = json.load(f)


def test_path():
    path = os.path.join(BASE_DIR, "acid.svg")
    base = base64.b64encode(bytes(resvg_py.svg_to_bytes(svg_path=path))).decode("utf-8")
    assert base == _expected["acid"]


def test_gzip_path():
    path = os.path.join(BASE_DIR, "acid.svg.gz")
    base = base64.b64encode(bytes(resvg_py.svg_to_bytes(svg_path=path))).decode("utf-8")
    assert base == _expected["acid"]

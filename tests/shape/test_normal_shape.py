import base64
import json
import os

from pathlib import Path

import resvg_py

BASE_DIR = Path(__file__).resolve().parent

with open(BASE_DIR / "expected_outputs.json", encoding="utf-8") as f:
    _expected = json.load(f)


def test_rectangle():
    path = os.path.join(BASE_DIR, "rectangle.svg")
    _base64 = base64.b64encode(
        bytes(resvg_py.svg_to_bytes(svg_path=path))
    ).decode("utf-8")
    assert _base64 == _expected["rectangle"]

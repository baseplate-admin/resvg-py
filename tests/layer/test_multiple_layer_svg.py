import base64
import json

from pathlib import Path

import resvg_py

BASE_DIR = Path(__file__).resolve().parent

with open(BASE_DIR / "expected_outputs.json", encoding="utf-8") as f:
    _expected = json.load(f)


def test_multiple_layer_svg():
    path = BASE_DIR / "test.svg"
    svg_bytes = resvg_py.svg_to_bytes(svg_path=str(path))
    base = base64.b64encode(svg_bytes).decode("utf-8")
    assert base == _expected["multi_layer"]

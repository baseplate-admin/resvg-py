import base64
import json

from pathlib import Path

import resvg_py

BASE_DIR = Path(__file__).resolve().parent

with open(BASE_DIR / "expected_outputs.json", encoding="utf-8") as f:
    _expected = json.load(f)


def test_complex_camera():
    camera_svg = BASE_DIR.parent / "test_images" / "complex_camera.svg"
    assert (
        base64.b64encode(bytes(resvg_py.svg_to_bytes(svg_path=str(camera_svg)))).decode(
            "utf-8"
        )
        == _expected["complex_camera"]
    )


def test_complex_camera_with_background():
    camera_svg = BASE_DIR.parent / "test_images" / "complex_camera.svg"
    assert (
        base64.b64encode(
            bytes(resvg_py.svg_to_bytes(svg_path=str(camera_svg), background="#070519"))
        ).decode("utf-8")
        == _expected["complex_camera_with_background"]
    )

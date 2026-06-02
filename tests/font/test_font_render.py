import base64
import json
import os
import sys

from pathlib import Path

import pytest
import resvg_py

BASE_DIR = Path(__file__).resolve().parent

with open(BASE_DIR / "expected_outputs.json", encoding="utf-8") as f:
    _expected = json.load(f)


@pytest.mark.skipif(
    sys.platform != "win32",
    reason="Runs only on Windows due to system font availability",
)
def test_with_system_font():
    path = os.path.join(BASE_DIR, "ink.svg")
    base = base64.b64encode(
        bytes(resvg_py.svg_to_bytes(svg_path=path)),
    ).decode("utf-8")
    assert base == _expected["with_system_font"]


def test_without_system_font():
    path = os.path.join(BASE_DIR, "ink.svg")
    base = base64.b64encode(
        bytes(resvg_py.svg_to_bytes(svg_path=path, skip_system_fonts=True))
    ).decode("utf-8")
    assert base == _expected["without_system_font"]


def test_with_kokoro_font():
    path = os.path.join(BASE_DIR, "ink.svg")
    font = str(Path(BASE_DIR, "font_dir", "Kokoro-Regular.ttf"))
    base = base64.b64encode(
        bytes(
            resvg_py.svg_to_bytes(
                svg_path=path, skip_system_fonts=True, font_files=[font]
            )
        )
    ).decode("utf-8")
    assert base == _expected["with_kokoro_font"]


def test_font_directory():
    path = os.path.join(BASE_DIR, "ink.svg")
    font = str(Path(BASE_DIR, "font_dir"))
    base = base64.b64encode(
        bytes(
            resvg_py.svg_to_bytes(
                svg_path=path, skip_system_fonts=True, font_dirs=[font]
            )
        )
    ).decode("utf-8")

    assert base == _expected["with_kokoro_font"]


def test_font_cache_consistency():
    path = os.path.join(BASE_DIR, "ink.svg")
    font = str(Path(BASE_DIR, "font_dir"))
    first = resvg_py.svg_to_bytes(svg_path=path, skip_system_fonts=True, font_dirs=[font])
    second = resvg_py.svg_to_bytes(svg_path=path, skip_system_fonts=True, font_dirs=[font])
    assert first == second

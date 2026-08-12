import base64
import json
import os

from pathlib import Path

import pytest
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


def test_corrupt_gzip_raises_value_error(tmp_path):
    path = tmp_path / "corrupt.svgz"
    path.write_bytes(b"\x1f\x8bnot gzip")

    with pytest.raises(ValueError) as exc_info:
        resvg_py.svg_to_bytes(svg_path=str(path))
    assert str(exc_info.value).startswith(f"Failed to decompress '{path}':")


def test_non_utf8_file_raises_value_error(tmp_path):
    path = tmp_path / "non-utf8.svg"
    path.write_bytes(b"\xff\xfe")

    with pytest.raises(ValueError) as exc_info:
        resvg_py.svg_to_bytes(svg_path=str(path))
    assert str(exc_info.value).startswith(f"'{path}' is not valid UTF-8:")


def test_unreadable_path_raises_value_error(tmp_path):
    path = tmp_path / "directory.svg"
    path.mkdir()

    with pytest.raises(ValueError) as exc_info:
        resvg_py.svg_to_bytes(svg_path=str(path))
    assert str(exc_info.value).startswith(f"Failed to read '{path}':")

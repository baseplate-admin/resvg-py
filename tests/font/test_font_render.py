import resvg_py
from pathlib import Path
import os

BASE_DIR = Path(__file__).resolve().parent


def test_font_cache_consistency():
    path = os.path.join(BASE_DIR, "ink.svg")
    font = str(Path(BASE_DIR, "font_dir"))
    first = resvg_py.svg_to_bytes(
        svg_path=path, skip_system_fonts=True, font_dirs=[font]
    )
    second = resvg_py.svg_to_bytes(
        svg_path=path, skip_system_fonts=True, font_dirs=[font]
    )
    assert first == second

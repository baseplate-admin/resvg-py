"""
Regenerate expected base64 outputs for all image comparison tests.

Renders each test SVG through resvg_py, encodes the PNG as base64,
and writes the results to expected_outputs.json files beside each test module.

Usage:
    python scripts/sync_image_to_json.py
"""

import base64
import json
import sys
from pathlib import Path

# Project root is the parent of this script's parent directory
PROJECT_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(PROJECT_ROOT))

import resvg_py  # noqa: E402


def render_and_encode(svg_string: str | None = None, svg_path: str | None = None, **kwargs) -> str:
    """Render SVG to PNG and return base64-encoded string."""
    png_bytes = resvg_py.svg_to_bytes(svg_string=svg_string, svg_path=svg_path, **kwargs)
    return base64.b64encode(bytes(png_bytes)).decode("utf-8")


def write_json(path: Path, data: dict[str, str]) -> None:
    """Write expected outputs to a JSON file."""
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"  -> {path}")


# ---------------------------------------------------------------------------
# Shape tests
# ---------------------------------------------------------------------------
def sync_shape_tests() -> None:
    print("Syncing shape tests ...")
    shape_dir = PROJECT_ROOT / "tests" / "shape"
    result: dict[str, str] = {}

    # Rectangle (inline SVG)
    rect_svg = shape_dir / "rectangle.svg"
    if rect_svg.exists():
        result["rectangle"] = render_and_encode(svg_path=str(rect_svg))
    else:
        print("  WARNING: rectangle.svg not found, skipping")

    # Complex camera
    camera_svg = PROJECT_ROOT / "tests" / "test_images" / "complex_camera.svg"
    if camera_svg.exists():
        result["complex_camera"] = render_and_encode(svg_path=str(camera_svg))
        result["complex_camera_with_background"] = render_and_encode(
            svg_path=str(camera_svg), background="#070519"
        )
    else:
        print("  WARNING: complex_camera.svg not found, skipping")

    write_json(shape_dir / "expected_outputs.json", result)


# ---------------------------------------------------------------------------
# Path tests
# ---------------------------------------------------------------------------
def sync_path_tests() -> None:
    print("Syncing path tests ...")
    path_dir = PROJECT_ROOT / "tests" / "path"
    result: dict[str, str] = {}

    acid_svg = path_dir / "acid.svg"
    if acid_svg.exists():
        result["acid"] = render_and_encode(svg_path=str(acid_svg))
    else:
        print("  WARNING: acid.svg not found, skipping")

    write_json(path_dir / "expected_outputs.json", result)


# ---------------------------------------------------------------------------
# Layer tests
# ---------------------------------------------------------------------------
def sync_layer_tests() -> None:
    print("Syncing layer tests ...")
    layer_dir = PROJECT_ROOT / "tests" / "layer"
    result: dict[str, str] = {}

    test_svg = layer_dir / "test.svg"
    if test_svg.exists():
        result["multi_layer"] = render_and_encode(svg_path=str(test_svg))
    else:
        print("  WARNING: test.svg not found, skipping")

    write_json(layer_dir / "expected_outputs.json", result)


# ---------------------------------------------------------------------------
# Font tests
# ---------------------------------------------------------------------------
def sync_font_tests() -> None:
    print("Syncing font tests ...")
    font_dir = PROJECT_ROOT / "tests" / "font"
    result: dict[str, str] = {}

    ink_svg = font_dir / "ink.svg"
    font_file = font_dir / "font_dir" / "Kokoro-Regular.ttf"

    if not ink_svg.exists():
        print("  WARNING: ink.svg not found, skipping font tests")
        return

    # With system fonts
    result["with_system_font"] = render_and_encode(svg_path=str(ink_svg))

    # Without system fonts
    result["without_system_font"] = render_and_encode(
        svg_path=str(ink_svg), skip_system_fonts=True
    )

    # With kokoro font file
    if font_file.exists():
        result["with_kokoro_font"] = render_and_encode(
            svg_path=str(ink_svg), skip_system_fonts=True, font_files=[str(font_file)]
        )
    else:
        print("  WARNING: Kokoro-Regular.ttf not found, skipping with_kokoro_font")

    write_json(font_dir / "expected_outputs.json", result)


# ---------------------------------------------------------------------------
def main() -> None:
    print("=== resvg-py: Sync image outputs ===\n")

    sync_shape_tests()
    sync_path_tests()
    sync_layer_tests()
    sync_font_tests()

    print("\nDone.")


if __name__ == "__main__":
    main()

import math
import struct

import pytest
import resvg_py


SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="80"><rect width="100" height="80"/></svg>'
TEXT_SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="80"><text x="5" y="20">Hi</text></svg>'


def png_size(png_bytes: bytes) -> tuple[int, int]:
    """Return the PNG dimensions from the IHDR header."""
    return struct.unpack(">II", png_bytes[16:24])


def test_float_zoom_scales_output_size():
    assert png_size(bytes(resvg_py.svg_to_bytes(svg_string=SVG, zoom=1.5))) == (150, 120)


def test_float_dpi_and_font_size_are_accepted():
    png_bytes = bytes(resvg_py.svg_to_bytes(svg_string=TEXT_SVG, dpi=192.5, font_size=12.5))
    assert png_size(png_bytes) == (100, 80)


@pytest.mark.parametrize(
    ("argument", "value"),
    [
        ("zoom", 0.0),
        ("zoom", -1.0),
        ("zoom", math.inf),
        ("dpi", -1.0),
        ("dpi", math.nan),
        ("font_size", 0.0),
        ("font_size", -1.0),
        ("font_size", math.inf),
    ],
)
def test_invalid_float_options_raise_value_error(argument, value):
    with pytest.raises(ValueError):
        resvg_py.svg_to_bytes(svg_string=SVG, **{argument: value})

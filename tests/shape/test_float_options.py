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
def test_invalid_numeric_options_raise_value_error(argument, value):
    with pytest.raises(ValueError):
        resvg_py.svg_to_bytes(svg_string=SVG, **{argument: value})


@pytest.mark.parametrize(
    "kwargs",
    [
        {"width": 0},
        {"height": 0},
        {"width": 0, "height": 1},
        {"width": 1, "height": 0},
    ],
)
def test_zero_dimension_raises_value_error(kwargs):
    with pytest.raises(ValueError):
        resvg_py.svg_to_bytes(svg_string=SVG, **kwargs)


@pytest.mark.parametrize("argument", ["width", "height"])
@pytest.mark.parametrize("value", [-1, 2**32])
def test_dimension_outside_u32_raises_overflow_error(argument, value):
    with pytest.raises(OverflowError):
        resvg_py.svg_to_bytes(svg_string=SVG, **{argument: value})


@pytest.mark.parametrize(
    ("kwargs", "expected_size"),
    [
        ({"width": 50, "zoom": math.nan}, (50, 40)),
        ({"height": 40, "zoom": math.nan}, (50, 40)),
        ({"width": 30, "height": 20, "zoom": math.nan}, (25, 20)),
    ],
)
def test_dimension_takes_precedence_over_zoom(kwargs, expected_size):
    assert png_size(bytes(resvg_py.svg_to_bytes(svg_string=SVG, **kwargs))) == expected_size


def test_arguments_are_converted_before_dimension_precedence():
    with pytest.raises(TypeError):
        resvg_py.svg_to_bytes(svg_string=SVG, width=50, zoom="invalid")

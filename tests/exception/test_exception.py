import pytest
import resvg_py


SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="4" height="4"/>'


def test_broken_svg_raises_exception():
    broken_svg = """<svg xmlns="http://www.w3.org/2000/svg" width=1194" height="240" viewBox="0 0 1194">
    <g>
      <path d="M240 219C240 227.24 233.24 234 225 234H15C6.76 234 0 227.24 0 219V129C0 120.76 4.78 109.22 10.6 103.4L109.4 4.61999C115.24 -1.22001 124.78 -1.22001 130.62 4.61999L229.4 103.4C235.24 109.24 240 120.76 240 129V219Z" />
    </g>
  </svg>"""
    
    with pytest.raises(ValueError):
        resvg_py.svg_to_bytes(svg_string=broken_svg)


@pytest.mark.parametrize("argument", ["shape_rendering", "text_rendering", "image_rendering"])
def test_rendering_policy_none_raises_type_error(argument):
    with pytest.raises(TypeError):
        resvg_py.svg_to_bytes(svg_string=SVG, **{argument: None})

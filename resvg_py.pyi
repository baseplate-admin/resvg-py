from typing import Literal


__version__ : str

def svg_to_bytes(
    svg_string: str | None = None,
    svg_path: str | None = None,
    background: str | None = None,
    skip_system_fonts: bool | None = False,
    log_information: bool = False,
    width: int | None = None,
    height: int | None = None,
    zoom: int | None = None,
    dpi: int | None = 0,
    resources_dir: str | None = None,
    languages: list[str] | None = [],
    font_size: int | None = 16,
    font_family: str | None = Literal["Times New Roman"],
    serif_family: str | None = Literal["Times New Roman"],
    sans_serif_family: str | None = Literal["Arial"],
    cursive_family: str | None = Literal["Comic Sans MS"],
    fantasy_family: str | None = ["Impact"],
    monospace_family: str | None = Literal["Courier New"],
    font_files: list[str] | None = None,
    font_dirs: list[str] | None = None,
    shape_rendering: Literal[
        "optimize_speed", "crisp_edges", "geometric_precision"
    ] = Literal["geometric_precision"],
    text_rendering: Literal[
        "optimize_speed", "optimize_legibility", "optimize_legibility"
    ] = Literal["optimize_legibility"],
    image_rendering: Literal["optimize_quality", "optimize_speed"] = Literal[
        "optimize_quality"
    ],
) -> list[bytes]:
    """
    :param svg_str: A string containing valid svg.
    :param svg_path: A path to a valid svg.
    :param width: An Integer containing the pixels size for width.
    :param height: An Integer containing the pixels size for height.
    :param zoom: An Integer containing the zoom percentage.
    :param dpi: An Integer containing DPI size for the svg rendering.
    :param resources_dir: A directory that contains resources for svg rendering. Such as `foreign objects <https://developer.mozilla.org/en-US/docs/Web/SVG/Element/foreignObject>`_.
    :param languages: A list of string containing the languages used for `svg` rendering
    :param font_size: An integer describing the font_size.
    :param font_family: A string that describes the font family used in SVG.
    :param serif_family: A string that describes the serif font family used in SVG.
    :param sans_serif_family: A string that describes the sans serif font family used in SVG.
    :param cursive_family: A string that describes the cursive font family used in SVG.
    :param fantasy_family: A string that describes the fantasy font family used in SVG.
    :param monospace_family: A string that describes the monospace font family used in SVG.
    :param font_files: A list of paths that contain the font file.
    :param font_dirs: A list of directories that contain the font file. This parameter will add all the present files in the directory.
    :param shape_rendering: The `shape rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/shape-rendering>`_ used in resvg. **Defaults to "geometric_precision"**.
    :param text_rendering: The `text rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-rendering>`_ used in resvg. **Defaults to "optimize_legibility"**.
    :param image_rendering: The `image rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/image-rendering>`_ used in resvg. **Defaults to "optimize_quality"**.
    :param background: A `CSS color <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value>`_ value that describes the canvas size.
    """

    ...

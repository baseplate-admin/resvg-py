from typing import Literal

def svg_to_bytes(
    svg_string: str | None = None,
    svg_path: str | None = None,
    width: int | None = None,
    height: int | None = None,
    resources_dir: str | None = None,
    languages: list[str] | None = None,
    font_size: int | None = None,
    font_family: str | None = None,
    serif_family: str | None = None,
    sans_serif_family: str | None = None,
    cursive_family: str | None = None,
    fantasy_family: str | None = None,
    monospace_family: str | None = None,
    font_files: list[str] | None = None,
    font_dirs: list[str] | None = None,
    shape_rendering: Literal["optimize_speed"]
    | Literal["crisp_edges"]
    | Literal["geometric_precision"] = Literal["geometric_precision"],
    text_rendering: Literal["optimize_speed"]
    | Literal["optimize_legibility"]
    | Literal["geometric_precision"] = Literal["optimize_legibility"],
    image_rendering: Literal["optimize_quality"] | Literal["optimize_speed"] = Literal[
        "optimize_quality"
    ],
    background: str | None = None,
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

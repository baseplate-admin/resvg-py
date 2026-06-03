from typing import Literal

__version__: str
__author__: str
__resvg_version__: str

def svg_to_bytes(
    svg_string: str | None = None,
    svg_path: str | None = None,
    background: str | None = None,
    skip_system_fonts: bool = False,
    log_information: bool = False,
    width: int | None = None,
    height: int | None = None,
    zoom: int | None = None,
    dpi: int = 0,
    style_sheet: str | None = None,
    resources_dir: str | None = None,
    languages: list[str] | None = None,
    font_size: int = 16,
    font_family: str | None = None,
    serif_family: str | None = None,
    sans_serif_family: str | None = None,
    cursive_family: str | None = None,
    fantasy_family: str | None = None,
    monospace_family: str | None = None,
    font_files: list[str] | None = None,
    font_dirs: list[str] | None = None,
    shape_rendering: Literal[
        "optimize_speed", "crisp_edges", "geometric_precision"
    ] = "geometric_precision",
    text_rendering: Literal[
        "optimize_speed", "optimize_legibility", "geometric_precision"
    ] = "optimize_legibility",
    image_rendering: Literal["optimize_quality", "optimize_speed"] = "optimize_quality",
) -> bytes: ...

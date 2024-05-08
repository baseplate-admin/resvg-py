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
    """"""

    ...

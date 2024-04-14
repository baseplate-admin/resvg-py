Resvg Module
============

.. currentmodule:: resvg_py.svg_to_base64

.. function:: svg_to_base64(svg_string:str,width:int|None=None,height:int|None=None,zoom:int|None=None,dpi:int|None=None,resources_dir:width:str|None=None,languages:list[str]|None=None,font_size:float|None=None,font_family:str|None=None,serif_family:str|None=None,sans_serif_family:str|None=None,cursive_family:str|None=None,fantasy_family:str|None=None,monospace_family:str|None=None,font_files:list[str]|None=None,font_dirs:list[str]|None=None,shape_rendering:"optimize_speed"|"crisp_edges"|"geometric_precision"="geometric_precision",text_rendering:"optimize_speed"|"optimize_legibility"|"geometric_precision"="optimize_legibility",image_rendering:"optimize_quality"|"optimize_speed"="optimize_quality",background:str|None=None) -> str

    :param svg_string: A string containing valid svg.
    :type svg_string: string

    :param width: An Integer containing the pixels size for width.
    :type width: int or None

    :param height: An Integer containing the pixels size for height.
    :type height: int or None

    :param zoom: An Integer containing the zoom percentage.
    :type zoom: int or None
    
    :param dpi: An Integer containing DPI size for the svg rendering.
    :type dpi: int or None

    :param resources_dir: A directory that contains resources for svg rendering. Such as `foreign objects <https://developer.mozilla.org/en-US/docs/Web/SVG/Element/foreignObject>`_.
    :type resources_dir: string or None

    :param languages: A list of string containing the languages used for `svg` rendering
    :type languages: list[str] or None

    :param font_size: An integer describing the font_size.
    :type font_size: float or None

    :param font_family: A string that describes the font family used in SVG.
    :type font_family: str or None

    :param serif_family: A string that describes the serif font family used in SVG.
    :type serif_family: str or None

    :param sans_serif_family: A string that describes the sans serif font family used in SVG.
    :type sans_serif_family: str or None

    :param cursive_family: A string that describes the cursive font family used in SVG.
    :type cursive_family: str or None

    :param fantasy_family: A string that describes the fantasy font family used in SVG.
    :type fantasy_family: str or None

    :param monospace_family: A string that describes the monospace font family used in SVG.
    :type monospace_family: str or None

    :param font_files: A list of paths that contain the font file.
    :type font_files: list[str] or None

    :param font_dirs: A list of directories that contain the font file. This parameter will add all the present files in the directory.
    :type font_dirs: list[str] or None

    :param shape_rendering: The `shape rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/shape-rendering>`_ used in resvg. **Defaults to "geometric_precision"**.
    :type shape_rendering: "optimize_speed" or "crisp_edges" or **"geometric_precision"**

    :param text_rendering: The `text rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-rendering>`_ used in resvg. **Defaults to "optimize_legibility"**.
    :type text_rendering: "optimize_speed" or **"optimize_legibility"** or "geometric_precision"

    :param image_rendering: The `image rendering method <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/image-rendering>`_ used in resvg. **Defaults to "optimize_quality"**.
    :type image_rendering: **"optimize_quality"** or "optimize_speed"

    :param background: A `CSS color <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value>`_ value that describes the canvas size.
    :type background: str or None 

    :return: base64 encoded string.
    :rtype: str
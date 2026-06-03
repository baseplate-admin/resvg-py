API Reference
=============

.. currentmodule:: resvg_py

Module attributes
-----------------

.. data:: __version__

   :iconify:`mdi:tag` Package version string, e.g. ``"0.3.2"``.

   :type: str

.. data:: __author__

   :iconify:`mdi:account` Package author.

   :type: str

.. data:: __resvg_version__

   :iconify:`mdi:package-variant` Version of the underlying ``resvg`` Rust crate.

   :type: str

svg_to_bytes
------------

.. autofunction:: svg_to_bytes

Parameter reference
~~~~~~~~~~~~~~~~~~~

.. list-table::
   :widths: 20 12 14 54
   :header-rows: 1

   * - Parameter
     - Type
     - Default
     - Description
   * - ``svg_string``
     - ``str | None``
     - ``None``
     - SVG markup as a UTF-8 string. Preferred over ``svg_path``.
   * - ``svg_path``
     - ``str | None``
     - ``None``
     - Path to an SVG file. ``.svgz`` auto-decompresses.
   * - ``background``
     - ``str | None``
     - ``None``
     - CSS color for the canvas background.
   * - ``skip_system_fonts``
     - ``bool``
     - ``False``
     - Skip loading system fonts. Provide ``font_files`` or ``font_dirs``.
   * - ``log_information``
     - ``bool``
     - ``False``
     - Enable resvg debug logging to stdout.
   * - ``width``
     - ``int | None``
     - ``None``
     - Target pixel width.
   * - ``height``
     - ``int | None``
     - ``None``
     - Target pixel height.
   * - ``zoom``
     - ``int | None``
     - ``None``
     - Integer zoom multiplier.
   * - ``dpi``
     - ``int``
     - ``0``
     - DPI override (``0`` = SVG default).
   * - ``style_sheet``
     - ``str | None``
     - ``None``
     - CSS stylesheet string applied during parse.
   * - ``resources_dir``
     - ``str | None``
     - ``None``
     - Directory for ``xlink:href`` resource references.
   * - ``languages``
     - ``list[str]``
     - ``[]``
     - Preferred languages for ``<switch>`` elements.
   * - ``font_size``
     - ``int``
     - ``16``
     - Default font size in pixels.
   * - ``font_family``
     - ``str | None``
     - OS-dependent
     - Default (generic) font family.
   * - ``serif_family``
     - ``str | None``
     - OS-dependent
     - Serif generic family override.
   * - ``sans_serif_family``
     - ``str | None``
     - OS-dependent
     - Sans-serif generic family override.
   * - ``cursive_family``
     - ``str | None``
     - OS-dependent
     - Cursive generic family override.
   * - ``fantasy_family``
     - ``str | None``
     - OS-dependent
     - Fantasy generic family override.
   * - ``monospace_family``
     - ``str | None``
     - OS-dependent
     - Monospace generic family override.
   * - ``font_files``
     - ``list[str] | None``
     - ``None``
     - Explicit font file paths to load.
   * - ``font_dirs``
     - ``list[str] | None``
     - ``None``
     - Directories recursively scanned for fonts.
   * - ``shape_rendering``
     - ``str``
     - ``"geometric_precision"``
     - Shape rendering policy.
   * - ``text_rendering``
     - ``str``
     - ``"optimize_legibility"``
     - Text rendering policy.
   * - ``image_rendering``
     - ``str``
     - ``"optimize_quality"``
     - Image rendering policy.

Rendering policies
~~~~~~~~~~~~~~~~~~

.. tabs::

   .. tab:: :iconify:`mdi:shape` shape_rendering

      **Default:** ``"geometric_precision"``

      .. list-table::
         :widths: 30 70
         :header-rows: 1

         * - Value
           - Behavior
         * - ``"geometric_precision"``
           - Highest quality. Full geometric accuracy.
         * - ``"crisp_edges"``
           - Sharp edges over curves. Good for pixel art.
         * - ``"optimize_speed"``
           - Fastest. May approximate curves.

   .. tab:: :iconify:`mdi:textbox` text_rendering

      **Default:** ``"optimize_legibility"``

      .. list-table::
         :widths: 30 70
         :header-rows: 1

         * - Value
           - Behavior
         * - ``"optimize_legibility"``
           - Balanced. May adjust kerning.
         * - ``"optimize_speed"``
           - Skip hinting for speed.
         * - ``"geometric_precision"``
           - Disable font hinting. Exact positions.

   .. tab:: :iconify:`mdi:image` image_rendering

      **Default:** ``"optimize_quality"``

      .. list-table::
         :widths: 30 70
         :header-rows: 1

         * - Value
           - Behavior
         * - ``"optimize_quality"``
           - High-quality upscaling filter.
         * - ``"optimize_speed"``
           - Nearest-neighbor or fast bilinear.

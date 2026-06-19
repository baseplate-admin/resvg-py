Usage
=====

.. currentmodule:: resvg_py

The core API is a single function, :py:func:`svg_to_bytes`, which renders SVG
markup to PNG bytes.

Rendering pipeline
------------------

.. mermaid::

   flowchart LR
       A[svg_string] --> P{parse XML}
       B[svg_path] --> P
       P --> T{has <text>?}
       T -->|yes| F[load fonts]
       T -->|no| S[skip fonts]
       F --> R[build usvg Tree]
       S --> R
       R --> D{dimensions}
       D -->|width| W[scale width]
       D -->|height| H[scale height]
       D -->|width+height| WH[fixed size]
       D -->|zoom| Z[multiply]
       D -->|none| O[original]
       W --> X[render pixmap]
       H --> X
       WH --> X
       Z --> X
       O --> X
       X --> E[encode PNG]
       E --> Out[bytes]

Basic usage
-----------

.. code-block:: python

   import resvg_py

   svg_string = """
   <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
       <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
   </svg>
   """

   png_bytes: bytes = resvg_py.svg_to_bytes(svg_string=svg_string)

.. note::

   ``svg_to_bytes`` returns **raw PNG bytes** -- not base64. Encode yourself
   if you need a data URI.

From a file
-----------

Pass a path instead of a string. ``.svgz`` (gzip-compressed) files are
automatically detected and decompressed.

.. code-block:: python

   png_bytes = resvg_py.svg_to_bytes(svg_path="/path/to/image.svg")

.. warning::

   If both ``svg_string`` and ``svg_path`` are provided, ``svg_string`` takes
   precedence. The path is only consulted when the string is empty or ``None``.

Resize at render time
---------------------

.. code-block:: python

   # fixed width and height
   resvg_py.svg_to_bytes(svg_string=svg, width=800, height=600)

   # width only -- height scales proportionally
   resvg_py.svg_to_bytes(svg_string=svg, width=1920)

   # height only -- width scales proportionally
   resvg_py.svg_to_bytes(svg_string=svg, height=1080)

   # zoom factor
   resvg_py.svg_to_bytes(svg_string=svg, zoom=1.5)

Base64 data URI
---------------

.. code-block:: python

   import base64

   png_bytes = resvg_py.svg_to_bytes(svg_string=svg)
   b64 = base64.b64encode(png_bytes).decode("utf-8")
   print(f"data:image/png;base64,{b64}")

Background color
----------------

.. code-block:: python

   resvg_py.svg_to_bytes(svg_string=svg, background="#f0f0f0")
   resvg_py.svg_to_bytes(svg_string=svg, background="rgba(255,0,0,0.5)")

.. tip::

   Any valid `CSS color
   <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value>`_ works --
   named colors, hex, rgb, hsl, all supported.

.. seealso::

   :doc:`api` -- Full parameter reference for every option.
   :doc:`fonts` -- Font configuration for text-heavy SVGs.

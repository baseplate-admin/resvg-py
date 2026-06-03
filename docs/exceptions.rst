Exceptions
==========

.. currentmodule:: resvg_py

All errors from ``resvg_py`` surface as :py:exc:`ValueError`. Below is every
scenario, the exact message, and how to avoid it.

Invalid SVG input
-----------------

.. code-block:: python

   ValueError: 'svg_string' is empty or 'svg_path' contains empty invalid svg

Raised when:

* ``svg_string`` is ``None``, ``""``, or only whitespace.
* ``svg_path`` points to a file that is empty, missing, or contains no valid SVG.
* A ``.svgz`` file fails to decompress.

.. tip::

   Validate your SVG with an XML parser or ``xmllint`` before passing it to
   ``svg_to_bytes``.

Invalid rendering option values
-------------------------------

Each rendering parameter accepts a fixed set of values. Passing anything else
raises:

.. code-block:: python

   ValueError: The value of 'shape_rendering' must be one of ...

.. list-table:: Valid values per parameter
   :widths: 25 75
   :header-rows: 1

   * - Parameter
     - Accepted values
   * - ``shape_rendering``
     - ``"optimize_speed"``, ``"crisp_edges"``, ``"geometric_precision"``
   * - ``text_rendering``
     - ``"optimize_speed"``, ``"optimize_legibility"``, ``"geometric_precision"``
   * - ``image_rendering``
     - ``"optimize_quality"``, ``"optimize_speed"``

.. warning::

   **Example -- wrong value**

   .. code-block:: python

      >>> resvg_py.svg_to_bytes(svg_string=svg, shape_rendering="best")
      ValueError: The value of 'shape_rendering' must be one of
      'optimize_speed','crisp_edges','geometric_precision'.
      It is currently 'best'

Background color parse error
----------------------------

.. code-block:: python

   ValueError: Error background: <svgtypes error details>

Raised when the ``background`` string cannot be parsed as a CSS color by the
``svgtypes`` crate.

.. warning::

   **Example -- invalid color**

   .. code-block:: python

      >>> resvg_py.svg_to_bytes(svg_string=svg, background="not-a-color")
      ValueError: Error background: ...

Rendering / processing errors
-----------------------------

Any failure during XML parsing, SVG tree construction, pixmap creation, or PNG
encoding is raised as ``ValueError`` with the underlying Rust error message.

Common causes:

* Malformed XML (unclosed tags, invalid namespaces)
* Dimensions that are zero or exceed the platform's maximum pixmap size
* Out-of-memory during PNG encoding

.. code-block:: python

   ValueError: <specific error message from the resvg library>

.. danger::

   Extremely large ``width`` / ``height`` or ``zoom`` values can cause an
   ``OutOfMemory`` panic at the Rust level. Keep dimensions under 32767 pixels
   per side.

.. seealso::

   :doc:`debugging` -- Enable logging to get more detail on failures.

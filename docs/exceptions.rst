Exceptions
==========

.. currentmodule:: resvg_py

The input and rendering failures described below surface as
:py:exc:`ValueError`. Python argument conversion can raise :py:exc:`TypeError`
or :py:exc:`OverflowError`; other filesystem options can raise subclasses of
:py:exc:`OSError`. An unforeseen unwinding Rust panic is translated by PyO3
into ``pyo3_runtime.PanicException`` instead of terminating the interpreter.

Invalid SVG input
-----------------

.. code-block:: python

   ValueError: 'svg_string' is empty or 'svg_path' contains empty invalid svg

Raised when:

* ``svg_string`` is ``None``, ``""``, or only whitespace.
* ``svg_path`` points to a file that is empty, missing, or contains no valid SVG.

File loading errors
-------------------

Failures while reading or decoding an existing ``svg_path`` include the path
and the underlying error:

.. code-block:: text

   ValueError: Failed to read '<path>': <OS error>
   ValueError: Failed to decompress '<path>': <gzip error>
   ValueError: '<path>' is not valid UTF-8: <UTF-8 error>

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

Invalid numeric values
----------------------

``width`` and ``height`` accept positive integers up to ``2**32 - 1``; zero
raises ``ValueError``, while values outside the accepted integer type fail
during Python argument conversion. After argument conversion, a supplied
dimension takes precedence over ``zoom`` value validation and scaling; zoom is
used only when both dimensions are absent. When used, ``zoom`` and
``font_size`` must be positive finite numbers; ``dpi`` must be a non-negative
finite number. Invalid values handled by ``svg_to_bytes`` raise ``ValueError``
naming the parameter and constraint.

Rendering / processing errors
-----------------------------

Any failure during XML parsing, SVG tree construction, pixmap creation, or PNG
encoding is raised as ``ValueError`` with the underlying Rust error message.

Common causes:

* Malformed XML (unclosed tags, invalid namespaces)
* Dimensions that exceed the platform's maximum pixmap size
* Out-of-memory during PNG encoding

.. code-block:: python

   ValueError: <specific error message from the resvg library>

.. danger::

   Extremely large ``width`` / ``height`` or ``zoom`` values can exhaust process
   memory. Keep dimensions under 32767 pixels per side.

.. seealso::

   :doc:`debugging` -- Enable logging to get more detail on failures.

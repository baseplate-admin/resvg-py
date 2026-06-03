Debugging
=========

.. currentmodule:: resvg_py

Enabling debug logging
----------------------

Pass ``log_information=True`` to :py:func:`svg_to_bytes` to print resvg's
internal warnings and errors to stdout:

.. code-block:: python

   import resvg_py

   png_bytes = resvg_py.svg_to_bytes(
       svg_string=svg,
       log_information=True,
   )

.. note::

   The logger is initialized once per process. Subsequent calls reuse the same
   logger -- the flag only needs to be ``True`` on the first call.

What gets logged
----------------

.. list-table::
   :widths: 15 85
   :header-rows: 1

   * - Level
     - When it fires
   * - :iconify:`mdi:alert-octagon` **Error**
     - Fatal parse or render failures
   * - :iconify:`mdi:alert` **Warning**
     - Missing fonts, skipped font files, empty font directories
   * - :iconify:`mdi:information` **Info**
     - General pipeline milestones
   * - :iconify:`mdi:bug` **Debug**
     - Per-node processing details

.. tip::

   If your SVG renders blank text boxes, enable logging -- you'll see a warning
   if no fonts were loaded for the requested family.

.. seealso::

   :doc:`exceptions` -- What errors surface as ``ValueError``.
   :doc:`fonts` -- How to load fonts so text renders correctly.

resvg_py
========

Safe Python bindings for `resvg`_, the Rust-based SVG renderer.

.. image:: https://img.shields.io/pypi/v/resvg-py?style=flat-square&labelColor=18181b&color=6366f1
   :target: https://pypi.org/project/resvg-py
   :alt: PyPI version
.. image:: https://img.shields.io/pypi/pyversions/resvg-py?style=flat-square&labelColor=18181b&color=6366f1
   :target: https://pypi.org/project/resvg-py
   :alt: Python versions
.. image:: https://img.shields.io/github/stars/baseplate-admin/resvg-py?style=flat-square&labelColor=18181b&color=6366f1&logo=github&logoColor=fff
   :target: https://github.com/baseplate-admin/resvg-py
   :alt: GitHub stars

.. tip::

   New here? Start with :doc:`installation` and :doc:`usage` to render your
   first SVG in under a minute.

Features
--------

**Blazing Fast**
  Zero-copy Rust rendering via ``resvg`` and ``usvg``. No Python GIL bottleneck.

**Full Control**
  Fonts, DPI, zoom, background, CSS stylesheets, and rendering policies.

**Pre-built Wheels**
  Ready for Linux, Windows, macOS, and Android — no Rust toolchain needed.

**One Function API**
  Call ``svg_to_bytes()`` and get PNG bytes. That's it.

Quick Start
-----------

.. code-block:: python

   import resvg_py

   png_bytes = resvg_py.svg_to_bytes(
       svg_string='<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="40" fill="#6366f1"/></svg>',
       width=200,
       height=200,
   )

   with open("output.png", "wb") as f:
       f.write(png_bytes)

Navigate
--------

**Getting Started**

- :doc:`installation` — Install and configure ``resvg_py``
- :doc:`usage` — Render SVGs with size, font, quality, and background controls

**Reference**

- :doc:`api` — Complete parameter reference and rendering pipeline
- :doc:`fonts` — System font defaults per OS and custom font loading
- :doc:`exceptions` — Every error scenario with exact messages
- :doc:`debugging` — Runtime logging and trace techniques

**More**

- :doc:`contributing` — Development setup and guidelines
- :doc:`license` — License information

.. toctree::
   :maxdepth: 2
   :caption: Contents
   :hidden:

   installation
   usage
   api
   fonts
   exceptions
   debugging
   contributing
   license

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

.. _resvg: https://docs.rs/resvg/latest/resvg/

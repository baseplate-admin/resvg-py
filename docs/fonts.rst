Fonts
=====

.. currentmodule:: resvg_py

``resvg_py`` uses platform-appropriate defaults and lets you override every
generic font family or load custom font files.

Default fonts per OS
--------------------

.. tabs::

   .. tab:: :iconify:`mdi:microsoft-windows` Windows / macOS

      .. list-table::
         :widths: 20 80
         :header-rows: 1

         * - Generic family
           - Default font
         * - Default (``font_family``)
           - `Times New Roman <https://en.wikipedia.org/wiki/Times_New_Roman>`_
         * - Serif (``serif_family``)
           - `Times New Roman <https://en.wikipedia.org/wiki/Times_New_Roman>`_
         * - Sans-serif (``sans_serif_family``)
           - `Arial <https://en.wikipedia.org/wiki/Arial>`_
         * - Cursive (``cursive_family``)
           - `Comic Sans MS <https://en.wikipedia.org/wiki/Comic_Sans>`_
         * - Fantasy (``fantasy_family``)
           - `Impact <https://en.wikipedia.org/wiki/Impact_(typeface)>`_
         * - Monospace (``monospace_family``)
           - `Courier New <https://en.wikipedia.org/wiki/Courier_(typeface)>`_

   .. tab:: :iconify:`mdi:linux` Linux

      .. list-table::
         :widths: 20 80
         :header-rows: 1

         * - Generic family
           - Default font
         * - Default (``font_family``)
           - `Liberation Serif <https://en.wikipedia.org/wiki/Liberation_fonts>`_
         * - Serif (``serif_family``)
           - `Liberation Serif <https://en.wikipedia.org/wiki/Liberation_fonts>`_
         * - Sans-serif (``sans_serif_family``)
           - `Liberation Sans <https://en.wikipedia.org/wiki/Liberation_fonts>`_
         * - Cursive (``cursive_family``)
           - `Comic Neue <https://en.wikipedia.org/wiki/Comic_Neue>`_
         * - Fantasy (``fantasy_family``)
           - Anton
         * - Monospace (``monospace_family``)
           - `Liberation Mono <https://en.wikipedia.org/wiki/Liberation_fonts>`_

.. note::

   On platforms outside Windows, macOS, and Linux, CSS generic names
   (``serif``, ``sans-serif``, ``cursive``, ``fantasy``, ``monospace``) are used
   as fallback family names.

Loading custom fonts
--------------------

.. tabs::

   .. tab:: :iconify:`mdi:file-outline` Single file

      .. code-block:: python

         resvg_py.svg_to_bytes(
             svg_string=svg,
             font_files=["/path/to/MyFont.ttf"],
         )

   .. tab:: :iconify:`mdi:folder-outline` Entire directory

      .. code-block:: python

         resvg_py.svg_to_bytes(
             svg_string=svg,
             font_dirs=["/path/to/fonts/"],
         )

   .. tab:: :iconify:`mdi:format-font` Override families

      .. code-block:: python

         resvg_py.svg_to_bytes(
             svg_string=svg,
             font_family="Inter",
             monospace_family="Fira Code",
         )

Skipping system fonts
---------------------

.. warning::

   Setting ``skip_system_fonts=True`` without providing ``font_files`` or
   ``font_dirs`` will cause text to render as empty boxes.

.. code-block:: python

   resvg_py.svg_to_bytes(
       svg_string=svg,
       skip_system_fonts=True,
       font_files=["/path/to/MyFont.ttf"],
   )

.. tip::

   Skip system fonts in containerized or CI environments where the host font
   database is incomplete or missing entirely.

.. seealso::

   :doc:`api` -- Full parameter list for all font-related arguments.

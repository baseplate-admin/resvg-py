Contributing
============

Project goals
-------------

* Expose every ``resvg`` feature in-memory -- nothing writes to disk.
* Minimal dependency count on both the Python and Rust sides.
* Maximum usability for the end developer.

Prerequisites
-------------

* Install `uv <https://astral.sh/uv>`_
* Rust toolchain (``rustc``, ``cargo``)
* Python 3.10+

Development workflow
--------------------

1. Sync dependencies:

   .. code-block:: bash

      uv sync

2. Make your changes to ``src/rust/lib.rs``.

3. Add or update tests under ``tests/``.

4. Run the test suite:

   .. code-block:: bash

      pytest .

5. Open a pull request.

.. tip::

   Rendering tests compare base64-encoded PNG output against pre-computed
   expected values in ``expected_outputs.json``. Use
   ``scripts/sync_image_to_json.py`` to regenerate expected outputs after an
   intentional visual change.

Contributors
------------

.. container:: rounded-image

   .. contributors:: baseplate-admin/resvg-py
      :avatars:



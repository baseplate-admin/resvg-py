Installation
============

Requirements
------------

.. list-table:: Supported platforms
   :widths: 25 25 50
   :header-rows: 1

   * - OS
     - Arch
     - Notes
   * - Linux
     - x86\_64, x86, aarch64, armv7, s390x, ppc64le
     - manylinux **and** musllinux wheels
   * - Windows
     - x64, x86, ARM64
     - Pre-built wheels on PyPI
   * - macOS
     - x86\_64, ARM64
     - Universal-compatible wheels
   * - Android
     - aarch64, x86\_64
     - Built via CI

.. important::

   Python **3.10** through **3.14** is supported. Python 3.9 wheels are no
   longer built -- upgrade your interpreter or install from source.

Install
-------

.. tabs::

   .. tab:: :iconify:`logos:pypi` pip

      .. code-block:: bash

         python -m pip install resvg_py

      That's it. No system libraries, no build step -- just a pre-compiled wheel.

   .. tab:: :iconify:`simple-icons:astral` uv

      .. code-block:: bash

         uv pip install resvg_py

      Or add as a project dependency:

      .. code-block:: bash

         uv add resvg_py

   .. tab:: :iconify:`logos:poetry` Poetry

      .. code-block:: bash

         poetry add resvg_py

      Or install in a virtual environment without adding to ``pyproject.toml``:

      .. code-block:: bash

         poetry run pip install resvg_py

   .. tab:: :iconify:`mdi:speedometer` PDM

      .. code-block:: bash

         pdm add resvg_py

   .. tab:: :iconify:`simple-icons:pipenv` Pipenv

      .. code-block:: bash

         pipenv install resvg_py

   .. tab:: :iconify:`logos:anaconda` Conda

      .. code-block:: bash

         conda install -c conda-forge resvg_py

      .. note::

         Conda-forge packages may lag behind the latest PyPI release by a few
         days. Use pip inside a conda env for the newest version:

         .. code-block:: bash

            conda create -n myenv python=3.12
            conda activate myenv
            pip install resvg_py

   .. tab:: :iconify:`mdi:terminal` Pipx

      .. code-block:: bash

         pipx install resvg_py

      .. warning::

         ``pipx`` is for CLI applications. ``resvg_py`` is a library -- use pip,
         uv, or Poetry instead.

.. tip::

   Pin a version in production regardless of your package manager:

   .. code-block:: bash

      pip install "resvg_py>=0.3,<1.0"
      uv pip install "resvg_py>=0.3,<1.0"
      poetry add "resvg_py>=0.3,<1.0"

Verify installation
-------------------

.. code-block:: python

   >>> import resvg_py
   >>> resvg_py.__version__
   '0.3.2'
   >>> resvg_py.__resvg_version__
   '0.47.0'

Install from source
-------------------

If no wheel is available for your platform, build from source with
`maturin <https://github.com/PyO3/maturin>`_:

.. tabs::

   .. tab:: :iconify:`simple-icons:rust` maturin

      .. code-block:: bash

         git clone https://github.com/baseplate-admin/resvg-py.git
         cd resvg-py
         maturin develop

   .. tab:: :iconify:`logos:pypi` pip

      .. code-block:: bash

         pip install git+https://github.com/baseplate-admin/resvg-py.git

   .. tab:: :iconify:`simple-icons:astral` uv

      .. code-block:: bash

         uv pip install git+https://github.com/baseplate-admin/resvg-py.git

.. toctree::
   :maxdepth: 1
   :caption: Next steps
   :hidden:

   usage

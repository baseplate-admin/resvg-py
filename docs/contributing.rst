Contributing
============

Thank you taking interest in this project.


Goals
-----

My goal for this project is:

* To enable all the features available in `resvg` but don't write to the disk, everything must be done in memory.
* Use the bare minimum amount of packages, in both python side and rust side
* Make the package as user friendly as possible

Getting Started
---------------

Pre-requisite packages:

* Install `poetry <https://python-poetry.org/>`_

* Install `pipx <https://pipx.pypa.io/stable/installation/>`_

* Install `maturin <https://www.maturin.rs/tutorial>`_


Then do the modifications to the `lib.rs` file and add test in tests directory.

1. Install **poetry** dependencies:
   
   .. code-block:: sh 
       
       poetry install

2. Activate **poetry** shell:

   .. code-block:: sh 
       
       poetry shell


3. Build with **maturin**:
   
   .. code-block:: sh
       
       maturin develop

4. Run tests:


   .. code-block:: sh

       pytest .


If all tests pass, please send a Pull Request to the main repository.
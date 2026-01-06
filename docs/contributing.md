# Contributing

Thank you taking interest in this project.

## Goals

My goal for this project is:

-   To enable all the features available in [resvg](https://github.com/razrfalcon/resvg) but don't write to the disk, everything must be done in memory.
-   Use the bare minimum amount of packages, in both python side and rust side
-   Make the package as user friendly as possible

## Getting Started

Pre-requisite packages:

-   Install [uv](https://astral.sh/uv)

Then do the modifications to the [lib.rs]{.title-ref} file and add test
in tests directory.

1.  Install **uv** dependencies:

    ```sh
    uv sync
    ```

2.  Run tests:

    ```sh
    pytest .
    ```

If all tests pass, please send a Pull Request to the main repository.

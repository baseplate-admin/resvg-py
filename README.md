# resvg_py

[![Downloads](https://static.pepy.tech/badge/resvg_py)](https://pepy.tech/project/resvg_py) [![CI](https://github.com/baseplate-admin/resvg-py/actions/workflows/CI.yaml/badge.svg?branch=master)](https://github.com/baseplate-admin/resvg-py/actions/workflows/CI.yaml) [![Documentation Status](https://readthedocs.org/projects/resvg-py/badge/?version=latest)](https://resvg-py.readthedocs.io/en/latest/?badge=latest)

A safe binding for the [resvg](https://github.com/RazrFalcon/resvg) project

## Install

```py
pip install resvg_py
```

Then use it like this:

```python

import resvg_py

svg_string = """
    <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
      <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
    </svg>
"""

print(resvg_py.svg_to_base64(svg_string))

```

## Requires

-   Python 3.8 or higher
Debugging
=========

While `resvg-py`_  is a very thin wrapper around the `resvg`_ project there might be bugs in *resvg-py* (or *resvg*).

In order to debug the issue you have to enable logging in `resvg-py`_

How to log in `resvg-py`_?

When you call `resvg-py`_'s function in your code you can pass `log_information=True` to print debug information to the stdout

For example:

.. code-block:: python

    import resvg_py
    import base64

    svg_string = """
        <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
          <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
        </svg>
    """

    # a large list of bytes
    png_bytes: list[bytes] = resvg_py.svg_to_bytes(
        svg_string=svg_string,
        log_information = True ## <----------- CHECK THIS LINE
    )
    base64_utf8_str = base64.b64encode(bytes(png_bytes)).decode("utf-8")
    print(f"data:image/png;base64,{base64_utf8_str}")




.. _resvg-py: https://github.com/baseplate-admin/resvg-py 
.. _resvg: https://docs.rs/resvg/latest/resvg/
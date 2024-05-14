# Debugging

While [resvg-py](https://github.com/baseplate-admin/resvg-py) is a very
thin wrapper around the [resvg](https://docs.rs/resvg/latest/resvg/)
project there might be bugs in _resvg-py_ (or _resvg_).

In order to debug the issue you have to enable logging in
[resvg-py](https://github.com/baseplate-admin/resvg-py)

How to log in [resvg-py](https://github.com/baseplate-admin/resvg-py)?

When you call [resvg-py](https://github.com/baseplate-admin/resvg-py)\'s
function in your code you can pass [log_information=True]{.title-ref} to
print debug information to the stdout

For example:

```python
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
```

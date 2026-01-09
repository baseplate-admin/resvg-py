# Debugging

Normally `resvg-py` throws exception on any kind of malformed svgs. But there can be undefined behavior when using `resvg-py`.

In order to debug these kind of issues, you can enable logging in [resvg-py](https://github.com/baseplate-admin/resvg-py). Please check the [Logging](#logging) section for more details.

# Logging

When you call [resvg-py](https://github.com/baseplate-admin/resvg-py)'s `svg_to_bytes` function in your code you can pass `log_information=True` to print debug information to the stdout.

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

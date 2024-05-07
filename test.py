import resvg_py

svg_string = """
    <svg width="300" height="130" xmlns="http://www.w3.org/2000/svg">
      <rect width="200" height="100" x="10" y="10" rx="20" ry="20" fill="blue" />
    </svg>
"""

import json
import base64


base64_utf8_str = base64.b64encode(
    bytes(resvg_py.svg_to_base64(svg_string=svg_string))
).decode("utf-8")
dataurl = f"data:image/png;base64,{base64_utf8_str}"

print(dataurl)

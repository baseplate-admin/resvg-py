import resvg_py


with open("ink.png", "wb") as f:
    f.write(resvg_py.svg_to_bytes(svg_path="ink.svg",))

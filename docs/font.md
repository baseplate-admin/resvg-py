# Fonts

`resvg_py` comes with sensible defaults for fonts across different operating systems. It also allows you to load your own fonts or font directories.

## Default Fonts

Depending on your operating system, `resvg_py` uses different default fonts if none are specified.

### Windows / MacOS

| Font Type      | Font Family     |
| :------------- | :-------------- |
| **Generic**    | Times New Roman |
| **Serif**      | Times New Roman |
| **Sans-serif** | Arial           |
| **Cursive**    | Comic Sans MS   |
| **Fantasy**    | Impact          |
| **Monospace**  | Courier New     |

### Linux

| Font Type      | Font Family      |
| :------------- | :--------------- |
| **Generic**    | Liberation Serif |
| **Serif**      | Liberation Serif |
| **Sans-serif** | Liberation Sans  |
| **Cursive**    | Comic Neue       |
| **Fantasy**    | Anton            |
| **Monospace**  | Liberation Mono  |

## Custom Fonts

You can load custom fonts by providing paths to font files or directories containing fonts.

```python
import resvg_py

svg_string = "..." # your svg string

# load a specific font file
resvg_py.svg_to_bytes(
    svg_string=svg_string,
    font_files=["/path/to/MyFont.ttf"]
)

# load a directory of fonts
resvg_py.svg_to_bytes(
    svg_string=svg_string,
    font_dirs=["/path/to/fonts/dir"]
)
```

## System Fonts

By default, certain system fonts are loaded. You can control this behavior using `skip_system_fonts`.

```python
import resvg_py

# This is just an example variable
svg_string = "..."

resvg_py.svg_to_bytes(
    svg_string=svg_string,
    skip_system_fonts=True,
    font_files=["/path/to/MyFont.ttf"] # You likely want to provide your own fonts if skipping system ones
)
```

## Font Families

You can specify which font family to use for different generic font families.

-   `font_family`: The default font family to use.
-   `serif_family`
-   `sans_serif_family`
-   `cursive_family`
-   `fantasy_family`
-   `monospace_family`

```python
import resvg_py

# This is just an example variable
svg_string = "..."

resvg_py.svg_to_bytes(
    svg_string=svg_string,
    font_family="Arial",
    monospace_family="Fira Code"
)
```

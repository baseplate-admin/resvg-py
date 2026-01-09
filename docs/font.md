# Fonts

`resvg_py` comes with sensible defaults for fonts across different operating systems. It also allows you to load your own fonts or font directories.

## Default Fonts

Depending on your operating system, `resvg_py` uses different default fonts if none are specified.

### Windows / MacOS

| Font Type      | Font Family                                                       |
| :------------- | :---------------------------------------------------------------- |
| **Generic**    | [Times New Roman](https://en.wikipedia.org/wiki/Times_New_Roman)  |
| **Serif**      | [Times New Roman](https://en.wikipedia.org/wiki/Times_New_Roman)  |
| **Sans-serif** | [Arial](https://en.wikipedia.org/wiki/Arial)                      |
| **Cursive**    | [Comic Sans MS](https://en.wikipedia.org/wiki/Comic_Sans)         |
| **Fantasy**    | [Impact](<https://en.wikipedia.org/wiki/Impact_(typeface)>)       |
| **Monospace**  | [Courier New](<https://en.wikipedia.org/wiki/Courier_(typeface)>) |

### Linux

| Font Type      | Font Family                                                        |
| :------------- | :----------------------------------------------------------------- |
| **Generic**    | [Liberation Serif](https://en.wikipedia.org/wiki/Liberation_fonts) |
| **Serif**      | [Liberation Serif](https://en.wikipedia.org/wiki/Liberation_fonts) |
| **Sans-serif** | [Liberation Sans](https://en.wikipedia.org/wiki/Liberation_fonts)  |
| **Cursive**    | [Comic Neue](https://en.wikipedia.org/wiki/Comic_Neue)             |
| **Fantasy**    | Anton                                                              |
| **Monospace**  | [Liberation Mono](https://en.wikipedia.org/wiki/Liberation_fonts)  |

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

-   `font_family`: The default font family to use when no font family is specified in the SVG or when the specified font family is not available.
-   `serif_family`: The default serif font family (e.g., Times New Roman).
-   `sans_serif_family`: The default sans-serif font family (e.g., Arial).
-   `cursive_family`: The default cursive font family (e.g., Comic Sans MS).
-   `fantasy_family`: The default fantasy font family (e.g., Impact).
-   `monospace_family`: The default monospace font family (e.g., Courier New).

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

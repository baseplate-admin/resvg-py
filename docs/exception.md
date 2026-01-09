# Exceptions

This page lists the exceptions that can be raised by `resvg_py`.

## ValueError

`resvg_py` primarily raises `ValueError` when it encounters invalid input or errors during the rendering process.

### Invalid SVG Input

If the provided `svg_string` is empty, or if `svg_path` points to an empty or invalid file (or a file that results in an empty string after decompression), a `ValueError` is raised.

```python
ValueError: 'svg_string' is empty or 'svg_path' contains empty invalid svg
```

### Invalid Option Values

Certain options expect specific string values. If an invalid value is provided, a `ValueError` is raised.

**shape_rendering**:
Must be one of: `'optimize_speed'`, `'crisp_edges'`, `'geometric_precision'`.

**text_rendering**:
Must be one of: `'optimize_speed'`, `'optimize_legibility'`, `'geometric_precision'`.

**image_rendering**:
Must be one of: `'optimize_quality'`, `'optimize_speed'`.

### Background Color Parsing

If the `background` argument is provided but cannot be parsed as a valid color string (per `svgtypes` crate rules), a `ValueError` is raised.

```python
ValueError: Error background: <error_details>
```

### Rendering and Processing Errors

Errors occurring during the SVG parsing (XML parsing), tree construction (`usvg`), or rendering phases are captured and raised as `ValueError`.

Common causes include:

-   Malformed SVG XML.
-   Failures to calculate scaled sizes (e.g. invalid dimensions).
-   Failures to create the pixel map (e.g. dimensions too large or zero).
-   Failure to encode the result to PNG.

```python
ValueError: <specific_error_message_from_rust_library>
```

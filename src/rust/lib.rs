/*
Based on
* https://github.com/RazrFalcon/resvg/blob/master/crates/resvg/src/main.rs
* https://github.com/mrdotb/resvg_nif/blob/master/native/resvg/src/lib.rs
*/

use pyo3::prelude::*;
use resvg;

#[derive(Clone, Copy, PartialEq, Debug)]
enum FitTo {
    /// Keep original size.
    Original,
    /// Scale to width.
    Width(u32),
    /// Scale to height.
    Height(u32),
    /// Scale to size.
    Size(u32, u32),
    /// Zoom by factor.
    Zoom(f32),
}

impl FitTo {
    fn fit_to_size(&self, size: resvg::tiny_skia::IntSize) -> Option<resvg::tiny_skia::IntSize> {
        match *self {
            FitTo::Original => Some(size),
            FitTo::Width(w) => size.scale_to_width(w),
            FitTo::Height(h) => size.scale_to_height(h),
            FitTo::Size(w, h) => resvg::tiny_skia::IntSize::from_wh(w, h).map(|s| size.scale_to(s)),
            FitTo::Zoom(z) => size.scale_by(z),
        }
    }

    fn fit_to_transform(&self, size: resvg::tiny_skia::IntSize) -> resvg::tiny_skia::Transform {
        let size1 = size.to_size();
        let size2 = match self.fit_to_size(size) {
            Some(v) => v.to_size(),
            None => return resvg::tiny_skia::Transform::default(),
        };
        resvg::tiny_skia::Transform::from_scale(
            size2.width() / size1.width(),
            size2.height() / size1.height(),
        )
    }
}
struct Opts {
    //  font_size: u32,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
    background: Option<svgtypes::Color>,
    font_files: Option<Vec<String>>,
    font_dirs: Option<Vec<String>>,
    // skip_system_fonts: bool,
    // Abstract Classes
    fit_to: FitTo,
    usvg_opt: resvg::usvg::Options,
    // Renderers
}

fn load_fonts(options: &mut Opts, fontdb: &mut resvg::usvg::fontdb::Database) {
    if let Some(font_files) = &options.font_files {
        for path in font_files {
            if let Err(e) = fontdb.load_font_file(path) {
                println!("Failed to load '{}' cause {}.", path.to_string(), e);
            }
        }
    }

    if let Some(font_dirs) = &options.font_dirs {
        for path in font_dirs {
            fontdb.load_fonts_dir(path);
        }
    }

    let take_or =
        |family: Option<String>, fallback: &str| family.unwrap_or_else(|| fallback.to_string());

    fontdb.set_serif_family(take_or(options.serif_family.take(), "Times New Roman"));
    fontdb.set_sans_serif_family(take_or(options.sans_serif_family.take(), "Arial"));
    fontdb.set_cursive_family(take_or(options.cursive_family.take(), "Comic Sans MS"));
    fontdb.set_fantasy_family(take_or(options.fantasy_family.take(), "Impact"));
    fontdb.set_monospace_family(take_or(options.monospace_family.take(), "Courier New"));
}
fn svg_to_skia_color(color: svgtypes::Color) -> resvg::tiny_skia::Color {
    resvg::tiny_skia::Color::from_rgba8(color.red, color.green, color.blue, color.alpha)
}
fn render_svg(options: Opts, tree: &resvg::usvg::Tree) -> Result<resvg::tiny_skia::Pixmap, String> {
    let mut pixmap = resvg::tiny_skia::Pixmap::new(
        tree.size().to_int_size().width(),
        tree.size().to_int_size().height(),
    )
    .unwrap();

    if let Some(background) = options.background {
        pixmap.fill(svg_to_skia_color(background));
    }
    let ts = options.fit_to.fit_to_transform(tree.size().to_int_size());
    resvg::render(tree, ts, &mut pixmap.as_mut());

    Ok(pixmap)
}

fn resvg_magic(mut options: Opts, svg_string: String) -> Result<Vec<u8>, String> {
    let xml_tree = {
        let xml_opt = resvg::usvg::roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        };
        resvg::usvg::roxmltree::Document::parse_with_options(&svg_string, xml_opt)
            .map_err(|e| e.to_string())
    }
    .unwrap();
    let has_text_nodes = xml_tree
        .descendants()
        .any(|n| n.has_tag_name(("http://www.w3.org/2000/svg", "text")));

    let mut fontdb = resvg::usvg::fontdb::Database::new();
    if has_text_nodes {
        load_fonts(&mut options, &mut fontdb);
    }
    let tree = {
        resvg::usvg::Tree::from_xmltree(&xml_tree, &options.usvg_opt, &fontdb)
            .map_err(|e| e.to_string())
    }
    .unwrap();
    let img: Vec<u8> = render_svg(options, &tree).unwrap().encode_png().unwrap();
    Ok(img)
}

#[pyfunction]
#[pyo3(name = "svg_to_bytes")]
fn svg_to_bytes(
    svg_string: Option<String>,
    svg_path: Option<String>,
    // Control width, height, zoom, dpi
    width: Option<u32>,
    height: Option<u32>,
    zoom: Option<u32>,
    dpi: Option<u32>,
    // Resource Directory
    resources_dir: Option<String>,
    // Fonts
    languages: Option<Vec<String>>,
    font_size: Option<u32>,
    font_family: Option<String>,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
    font_files: Option<Vec<String>>,
    font_dirs: Option<Vec<String>>,
    // Effects based
    shape_rendering: Option<String>,
    text_rendering: Option<String>,
    image_rendering: Option<String>,
    // Background
    background: Option<String>,
) -> PyResult<Vec<u8>> {
    let mut _svg_string = String::new();

    if let Some(svg_string) = svg_string {
        _svg_string = svg_string;
    }

    // Only check for path if provided string is empty
    if _svg_string.is_empty() {
        if let Some(svg_path) = svg_path {
            if std::path::Path::new(&svg_path).exists() {
                let mut svg_data =
                    std::fs::read(&svg_path).expect("failed to open the provided file");
                if svg_data.starts_with(&[0x1f, 0x8b]) {
                    svg_data = resvg::usvg::decompress_svgz(&svg_data)
                        .expect("can't decompress the svg file");
                };
                _svg_string = std::str::from_utf8(&svg_data)
                    .expect("can't convert bytes to utf-8")
                    .to_owned();
            }
        }
    }

    if _svg_string.is_empty() {
        panic!("`svg_string` is empty or `svg_path` contains empty invalid svg");
    }

    let mut fit_to = FitTo::Original;
    let mut default_size = resvg::usvg::Size::from_wh(100.0, 100.0).unwrap();

    if let (Some(w), Some(h)) = (width, height) {
        default_size = resvg::usvg::Size::from_wh(w as f32, h as f32).unwrap();
        fit_to = FitTo::Size(w, h);
    } else if let Some(w) = width {
        default_size = resvg::usvg::Size::from_wh(w as f32, 100.0).unwrap();
        fit_to = FitTo::Width(w);
    } else if let Some(h) = height {
        default_size = resvg::usvg::Size::from_wh(100.0, h as f32).unwrap();
        fit_to = FitTo::Height(h);
    } else if let Some(z) = zoom {
        fit_to = FitTo::Zoom(z as f32);
    }

    let _shape_rendering = match shape_rendering
        .unwrap_or("geometric_precision".to_string())
        .as_ref()
    {
        "optimize_speed" => resvg::usvg::ShapeRendering::OptimizeSpeed,
        "crisp_edges" => resvg::usvg::ShapeRendering::CrispEdges,
        "geometric_precision" => resvg::usvg::ShapeRendering::GeometricPrecision,
        _ => panic!("Unexpected invalid token for shape rendering"),
    };

    let _text_rendering = match text_rendering
        .unwrap_or("optimize_legibility".to_string())
        .as_ref()
    {
        "optimize_speed" => resvg::usvg::TextRendering::OptimizeSpeed,
        "optimize_legibility" => resvg::usvg::TextRendering::OptimizeLegibility,
        "geometric_precision" => resvg::usvg::TextRendering::GeometricPrecision,
        _ => panic!("Unexpected invalid token for text rendering"),
    };

    let _image_rendering = match image_rendering
        .unwrap_or("optimize_quality".to_string())
        .as_ref()
    {
        "optimize_quality" => resvg::usvg::ImageRendering::OptimizeQuality,
        "optimize_speed" => resvg::usvg::ImageRendering::OptimizeSpeed,
        _ => panic!("Unexpected invalid token for image rendering",),
    };

    let _resources_dir = match resources_dir {
        Some(value) => Some(std::fs::canonicalize(value).unwrap()),
        None => None,
    };

    let _background = match background {
        Some(color_str) => match color_str.parse::<svgtypes::Color>() {
            Ok(color) => Some(color),
            Err(error) => panic!("Error background: {}", error),
        },
        None => None,
    };

    let usvg_options = resvg::usvg::Options {
        resources_dir: _resources_dir,
        dpi: dpi.unwrap_or(0) as f32,
        font_family: font_family.unwrap_or_else(|| "Times New Roman".to_string()),
        font_size: font_size.unwrap_or(16) as f32,
        languages: languages.unwrap_or(vec![]),
        shape_rendering: _shape_rendering,
        text_rendering: _text_rendering,
        image_rendering: _image_rendering,
        default_size,
        image_href_resolver: resvg::usvg::ImageHrefResolver::default(),
    };

    let options = Opts {
        usvg_opt: usvg_options,
        background: _background,
        fit_to,
        serif_family,
        sans_serif_family,
        cursive_family,
        fantasy_family,
        monospace_family,
        font_files,
        font_dirs,
    };
    let pixmap = resvg_magic(options, _svg_string.trim().to_owned()).unwrap();
    Ok(pixmap)
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(svg_to_bytes, m)?)?;
    Ok(())
}

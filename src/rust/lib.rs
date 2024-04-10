/*
Based on
* https://github.com/RazrFalcon/resvg/blob/master/crates/resvg/src/main.rs
* https://github.com/mrdotb/resvg_nif/blob/master/native/resvg/src/lib.rs
*/

use base64::{engine::general_purpose, Engine as _};
use pyo3::prelude::*;
use resvg;
use resvg::usvg;

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
    fn fit_to_size(&self, size: tiny_skia::IntSize) -> Option<tiny_skia::IntSize> {
        match *self {
            FitTo::Original => Some(size),
            FitTo::Width(w) => size.scale_to_width(w),
            FitTo::Height(h) => size.scale_to_height(h),
            FitTo::Size(w, h) => tiny_skia::IntSize::from_wh(w, h).map(|s| size.scale_to(s)),
            FitTo::Zoom(z) => size.scale_by(z),
        }
    }

    fn fit_to_transform(&self, size: tiny_skia::IntSize) -> tiny_skia::Transform {
        let size1 = size.to_size();
        let size2 = match self.fit_to_size(size) {
            Some(v) => v.to_size(),
            None => return tiny_skia::Transform::default(),
        };
        tiny_skia::Transform::from_scale(
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

    font_files: Option<Vec<String>>,
    font_dirs: Option<Vec<String>>,
    // skip_system_fonts: bool,
    // Abstract Classes
    fit_to: FitTo,
    usvg_opt: usvg::Options,
    // Renderers
}

fn load_fonts(options: &mut Opts, fontdb: &mut usvg::fontdb::Database) {
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

fn render_svg(options: Opts, tree: &usvg::Tree) -> Result<tiny_skia::Pixmap, String> {
    let mut pixmap = tiny_skia::Pixmap::new(
        tree.size().to_int_size().width(),
        tree.size().to_int_size().height(),
    )
    .unwrap();
    let ts = options.fit_to.fit_to_transform(tree.size().to_int_size());
    resvg::render(tree, ts, &mut pixmap.as_mut());

    Ok(pixmap)
}

fn resvg_magic(mut options: Opts, svg_string: String) -> Result<Vec<u8>, String> {
    let xml_tree = {
        let xml_opt = usvg::roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        };
        usvg::roxmltree::Document::parse_with_options(&svg_string, xml_opt)
            .map_err(|e| e.to_string())
    }
    .unwrap();
    let has_text_nodes = xml_tree
        .descendants()
        .any(|n| n.has_tag_name(("http://www.w3.org/2000/svg", "text")));

    let mut fontdb = usvg::fontdb::Database::new();
    if has_text_nodes {
        load_fonts(&mut options, &mut fontdb);
    }
    let tree = {
        usvg::Tree::from_xmltree(&xml_tree, &options.usvg_opt, &fontdb).map_err(|e| e.to_string())
    }
    .unwrap();
    let img: Vec<u8> = render_svg(options, &tree).unwrap().encode_png().unwrap();
    Ok(img)
}

#[pyfunction]
fn svg_to_base64(
    svg_string: String,
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
) -> PyResult<String> {
    let mut fit_to = FitTo::Original;
    let mut default_size = usvg::Size::from_wh(100.0, 100.0).unwrap();

    if let (Some(w), Some(h)) = (width, height) {
        default_size = usvg::Size::from_wh(w as f32, h as f32).unwrap();
        fit_to = FitTo::Size(w, h);
    } else if let Some(w) = width {
        default_size = usvg::Size::from_wh(w as f32, 100.0).unwrap();
        fit_to = FitTo::Width(w);
    } else if let Some(h) = height {
        default_size = usvg::Size::from_wh(100.0, h as f32).unwrap();
        fit_to = FitTo::Height(h);
    } else if let Some(z) = zoom {
        fit_to = FitTo::Zoom(z as f32);
    }

    let _shape_rendering = match shape_rendering
        .unwrap_or("geometric_precision".to_string())
        .as_ref()
    {
        "optimize_speed" => usvg::ShapeRendering::OptimizeSpeed,
        "crisp_edges" => usvg::ShapeRendering::CrispEdges,
        "geometric_precision" => usvg::ShapeRendering::GeometricPrecision,
        _ => panic!("Unexpected invalid token for shape rendering"),
    };

    let _text_rendering = match text_rendering
        .unwrap_or("geometric_precision".to_string())
        .as_ref()
    {
        "optimize_speed" => usvg::TextRendering::OptimizeSpeed,
        "optimize_legibility" => usvg::TextRendering::OptimizeLegibility,
        "geometric_precision" => usvg::TextRendering::GeometricPrecision,
        _ => panic!("Unexpected invalid token for text rendering"),
    };

    let _image_rendering = match image_rendering
        .unwrap_or("optimize_quality".to_string())
        .as_ref()
    {
        "optimize_quality" => usvg::ImageRendering::OptimizeQuality,
        "optimize_speed" => usvg::ImageRendering::OptimizeSpeed,
        _ => panic!("Unexpected invalid token for image rendering",),
    };

    let _resources_dir = match resources_dir {
        Some(value) => Some(std::fs::canonicalize(value).unwrap()),
        None => None,
    };

    let usvg_options = usvg::Options {
        resources_dir: _resources_dir,
        dpi: dpi.unwrap_or(0) as f32,
        font_family: font_family.unwrap_or_else(|| "Times New Roman".to_string()),
        font_size: font_size.unwrap_or(16) as f32,
        languages: languages.unwrap_or(vec![]),
        shape_rendering: _shape_rendering,
        text_rendering: _text_rendering,
        image_rendering: _image_rendering,
        default_size,
        image_href_resolver: usvg::ImageHrefResolver::default(),
    };

    let options = Opts {
        usvg_opt: usvg_options,
        fit_to,
        serif_family,
        sans_serif_family,
        cursive_family,
        fantasy_family,
        monospace_family,
        font_files,
        font_dirs,
    };
    let pixmap = resvg_magic(options, svg_string).unwrap();
    Ok(general_purpose::STANDARD.encode(&pixmap))
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(svg_to_base64, m)?)?;
    Ok(())
}

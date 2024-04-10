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
    fit_to: FitTo,
    font_family: Option<String>,
    font_size: u32,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
    font_files: Vec<String>,
    font_dirs: Vec<String>,
    skip_system_fonts: bool,
}

fn load_fonts(options: &mut Opts, fontdb: &mut usvg::fontdb::Database) {
    for path in &options.font_files {
        if let Err(e) = fontdb.load_font_file(path) {
            println!("Failed to load '{}' cause {}.", path.to_string(), e);
        }
    }

    for path in &options.font_dirs {
        fontdb.load_fonts_dir(path);
    }

    let take_or =
        |family: Option<String>, fallback: &str| family.unwrap_or_else(|| fallback.to_string());

    fontdb.set_serif_family(take_or(options.serif_family.take(), "Times New Roman"));
    fontdb.set_sans_serif_family(take_or(options.sans_serif_family.take(), "Arial"));
    fontdb.set_cursive_family(take_or(options.cursive_family.take(), "Comic Sans MS"));
    fontdb.set_fantasy_family(take_or(options.fantasy_family.take(), "Impact"));
    fontdb.set_monospace_family(take_or(options.monospace_family.take(), "Courier New"));
}

fn render_svg(mut options: Opts, tree: &usvg::Tree) -> Result<tiny_skia::Pixmap, String> {
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
        usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default(), &fontdb)
            .map_err(|e| e.to_string())
    }
    .unwrap();
    let img: Vec<u8> = render_svg(options, &tree).unwrap().encode_png().unwrap();
    Ok(img)
}

#[pyfunction]
fn svg_to_base64(
    svg_string: String,
    // Control width, height, zoom
    width: Option<u32>,
    height: Option<u32>,
    zoom: Option<u32>,
    // Fonts
    font_family: Option<String>,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
    font_files: Option<Vec<String>>,
    font_dirs: Option<Vec<String>>,
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

    let options = Opts {
        fit_to,
        font_family: font_family,
        font_size: todo!(),
        serif_family,
        sans_serif_family,
        cursive_family,
        fantasy_family,
        monospace_family,
        font_files: font_files.unwrap(),
        font_dirs: font_dirs.unwrap(),
        skip_system_fonts: todo!(),
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

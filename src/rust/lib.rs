/*
Based on
* https://github.com/RazrFalcon/resvg/blob/master/crates/resvg/src/main.rs
* https://github.com/mrdotb/resvg_nif/blob/master/native/resvg/src/lib.rs
*/

use pyo3::{exceptions::PyValueError, prelude::*};
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

/// A simple stderr logger.
static LOGGER: SimpleLogger = SimpleLogger;
struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::LevelFilter::Warn
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            let line = record.line().unwrap_or(0);
            let args = record.args();

            match record.level() {
                log::Level::Error => println!("Error (in {}:{}): {}", target, line, args),
                log::Level::Warn => println!("Warning (in {}:{}): {}", target, line, args),
                log::Level::Info => println!("Info (in {}:{}): {}", target, line, args),
                log::Level::Debug => println!("Debug (in {}:{}): {}", target, line, args),
                log::Level::Trace => println!("Trace (in {}:{}): {}", target, line, args),
            }
        }
    }

    fn flush(&self) {}
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
    // Abstract Classes
    fit_to: FitTo,
    usvg_opt: resvg::usvg::Options,
    // Renderers
    skip_system_fonts: bool,
}

fn load_fonts(options: &mut Opts, fontdb: &mut resvg::usvg::fontdb::Database) {
    if let Some(font_files) = &options.font_files {
        for path in font_files {
            if let Err(e) = fontdb.load_font_file(path) {
                log::warn!("Failed to load '{}' cause {}.", path.to_string(), e);
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
    ).ok_or("Could not create pixmap".to_owned())?;

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
    }?;
    let has_text_nodes = xml_tree
        .descendants()
        .any(|n| n.has_tag_name(("http://www.w3.org/2000/svg", "text")));

    let mut fontdb = resvg::usvg::fontdb::Database::new();
    if !options.skip_system_fonts {
        fontdb.load_system_fonts();
    }

    if has_text_nodes {
        load_fonts(&mut options, &mut fontdb);
    }

    let tree = {
        resvg::usvg::Tree::from_xmltree(&xml_tree, &options.usvg_opt, &fontdb)
            .map_err(|e| e.to_string())
    }?;
    render_svg(options, &tree)?.encode_png().map_err(|_e| "Could not encode PNG".to_owned())
}

#[pyfunction]
#[pyo3(signature = ( 
    svg_string= None,
    svg_path = None,
    background = None,
    skip_system_fonts= false,
    log_information = false,
    width = None,
    height= None,
    zoom = None,
    dpi = 0,
    resources_dir = None,
    languages = vec![],
    font_size = 16,
    font_family = "Times New Roman".to_owned(),
    serif_family = "Times New Roman".to_owned(),
    sans_serif_family = "Arial".to_owned(),
    cursive_family = "Comic Sans MS".to_owned(),
    fantasy_family = "Impact".to_owned(),
    monospace_family = "Courier New".to_owned(),
    font_files = None,
    font_dirs = None,
    shape_rendering = "geometric_precision".to_owned(),
    text_rendering = "optimize_legibility".to_owned(),
    image_rendering = "optimize_quality".to_owned(),
    ))]
fn svg_to_bytes(
    svg_string: Option<String>,
    svg_path: Option<String>,
    // Background
    background: Option<String>,
    // Skip System Fonts
    skip_system_fonts: Option<bool>,
    // Log informations
    log_information: Option<bool>,
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
    sans_serif_family:Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family:Option<String>,
    font_files: Option<Vec<String>>,
    font_dirs: Option<Vec<String>>,
    // Effects based
    shape_rendering:Option<String>,
    text_rendering: Option<String>,
    image_rendering: Option<String>,

) -> PyResult<Vec<u8>> {
    if log_information.unwrap_or(false) {
        if let Ok(()) = log::set_logger(&LOGGER) {
            log::set_max_level(log::LevelFilter::Warn);
        }
    }

    let mut _svg_string = String::new();

    if let Some(svg_string) = svg_string {
        _svg_string = svg_string;
    }

    // Only check for path if provided string is empty
    if _svg_string.is_empty() {
        if let Some(svg_path) = svg_path {
            if std::path::Path::new(&svg_path).exists() {
                let mut svg_data =
                    std::fs::read(&svg_path).map_err(|_e| PyErr::new::<PyValueError, _>("failed to open the provided file"))?;
                if svg_data.starts_with(&[0x1f, 0x8b]) {
                    svg_data = resvg::usvg::decompress_svgz(&svg_data)
                        .map_err(|_e| PyErr::new::<PyValueError, _>("can't decompress the svg file"))?;
                };
                _svg_string = std::str::from_utf8(&svg_data)
                    .map_err(|_e| PyErr::new::<PyValueError, _>("can't convert bytes to utf-8"))?
                    .to_owned();
            }
        }
    }

    if _svg_string.is_empty() {
        return Err(PyErr::new::<PyValueError, _>("`svg_string` is empty or `svg_path` contains empty invalid svg"));
    }

    let mut fit_to = FitTo::Original;
    let mut default_size = resvg::usvg::Size::from_wh(100.0, 100.0).ok_or(PyErr::new::<PyValueError, _>("Could not build SVG default size"))?;

    if let (Some(w), Some(h)) = (width, height) {
        default_size = resvg::usvg::Size::from_wh(w as f32, h as f32).ok_or(PyErr::new::<PyValueError, _>("Could not build SVG size with width and height"))?;
        fit_to = FitTo::Size(w, h);
    } else if let Some(w) = width {
        default_size = resvg::usvg::Size::from_wh(w as f32, 100.0).ok_or(PyErr::new::<PyValueError, _>("Could not build SVG size with width"))?;
        fit_to = FitTo::Width(w);
    } else if let Some(h) = height {
        default_size = resvg::usvg::Size::from_wh(100.0, h as f32).ok_or(PyErr::new::<PyValueError, _>("Could not build SVG size with height"))?;
        fit_to = FitTo::Height(h);
    } else if let Some(z) = zoom {
        fit_to = FitTo::Zoom(z as f32);
    }

    let _shape_rendering_opt = match shape_rendering
        .unwrap_or("geometric_precision".to_string())
        .as_ref()
    {
        "optimize_speed" => Some(resvg::usvg::ShapeRendering::OptimizeSpeed),
        "crisp_edges" => Some(resvg::usvg::ShapeRendering::CrispEdges),
        "geometric_precision" => Some(resvg::usvg::ShapeRendering::GeometricPrecision),
        _ => None,
    };
    let _shape_rendering = _shape_rendering_opt.ok_or(PyErr::new::<PyValueError, _>("Unexpected invalid token for shape rendering"))?;

    let _text_rendering_opt = match text_rendering
        .unwrap_or("optimize_legibility".to_string())
        .as_ref()
    {
        "optimize_speed" => Some(resvg::usvg::TextRendering::OptimizeSpeed),
        "optimize_legibility" => Some(resvg::usvg::TextRendering::OptimizeLegibility),
        "geometric_precision" => Some(resvg::usvg::TextRendering::GeometricPrecision),
        _ => None,
    };
    let _text_rendering = _text_rendering_opt.ok_or(PyErr::new::<PyValueError, _>("Unexpected invalid token for text rendering"))?;

    let _image_rendering_opt = match image_rendering
        .unwrap_or("optimize_quality".to_string())
        .as_ref()
    {
        "optimize_quality" => Some(resvg::usvg::ImageRendering::OptimizeQuality),
        "optimize_speed" => Some(resvg::usvg::ImageRendering::OptimizeSpeed),
        _ => None,
    };
    let _image_rendering = _image_rendering_opt.ok_or(PyErr::new::<PyValueError, _>("Unexpected invalid token for image rendering"))?;

    let _resources_dir = match resources_dir {
        Some(value) => Some(std::fs::canonicalize(value)?),
        None => None,
    };

    let _background = match background {
        Some(color_str) => match color_str.parse::<svgtypes::Color>() {
            Ok(color) => Some(color),
            Err(_error) => None,
        },
        None => None,
    };

    let usvg_options = resvg::usvg::Options {
        resources_dir: _resources_dir,
        dpi: dpi.unwrap_or(0) as f32,
        font_family: font_family.unwrap_or("Times New Roman".to_string()),
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
        skip_system_fonts: skip_system_fonts.unwrap_or(false),
        fit_to,
        serif_family,
        sans_serif_family,
        cursive_family,
        fantasy_family,
        monospace_family,
        font_files,
        font_dirs,
    };
    let pixmap = resvg_magic(options, _svg_string.trim().to_owned());
    pixmap.map_err(|e| PyErr::new::<PyValueError, _>(e))
}

fn get_version() -> &'static str {
    static VERSION :  std::sync::OnceLock<String> =  std::sync::OnceLock::new();

    VERSION.get_or_init(||{
        env!("CARGO_PKG_VERSION").to_owned()
    })
}

fn get_author() -> &'static str {
    static AUTHOR : std::sync::OnceLock<String>  = std::sync::OnceLock::new();

    AUTHOR.get_or_init(||{
        
        env!("CARGO_PKG_AUTHORS").to_owned()
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__",get_version())?;
    m.add("__author__", get_author())?;
    m.add_function(wrap_pyfunction!(svg_to_bytes, m)?)?;
    Ok(())
}

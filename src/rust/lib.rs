/*
Based on
* https://github.com/RazrFalcon/resvg/blob/master/crates/resvg/src/main.rs
* https://github.com/mrdotb/resvg_nif/blob/master/native/resvg/src/lib.rs
*/

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use resvg::{self, usvg::{FontResolver}};
use core::panic;
use std::sync::Arc;

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
struct Opts<'a> {
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
    usvg_opt: resvg::usvg::Options<'a>,
    // Renderers
    skip_system_fonts: bool,
}

fn load_fonts(
    fontdb: &mut resvg::usvg::fontdb::Database,
    font_files: &Option<Vec<String>>,
    font_dirs: &Option<Vec<String>>,
    serif_family: String,
    sans_serif_family: String,
    cursive_family: String,
    fantasy_family: String,
    monospace_family: String,
) {
    if let Some(font_files) = font_files {
        for path in font_files {
            if let Err(e) = fontdb.load_font_file(path) {
                log::warn!("Failed to load '{}' cause {}.", path.to_string(), e);
            }
        }
    }

    if let Some(font_dirs) = font_dirs {
        for path in font_dirs {
            fontdb.load_fonts_dir(path);
        }
    }


    fontdb.set_serif_family(serif_family);
    fontdb.set_sans_serif_family(sans_serif_family);
    fontdb.set_cursive_family(cursive_family);
    fontdb.set_fantasy_family(fantasy_family);
    fontdb.set_monospace_family(monospace_family);

}

fn svg_to_skia_color(color: svgtypes::Color) -> resvg::tiny_skia::Color {
    resvg::tiny_skia::Color::from_rgba8(color.red, color.green, color.blue, color.alpha)
}

fn render_svg(
    background: Option<svgtypes::Color>,
    fit_to: FitTo,
    tree: &resvg::usvg::Tree
) -> Result<resvg::tiny_skia::Pixmap, String> {
    let original_size = tree.size().to_int_size();

    let final_size = fit_to.fit_to_size(original_size)
        .ok_or("Failed to calculate scaled size")?;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(
        final_size.width(),
        final_size.height(),
    ).ok_or("Failed to create pixmap")?;

    if let Some(background) = background {
        pixmap.fill(svg_to_skia_color(background));
    }

    let ts = fit_to.fit_to_transform(original_size);
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

    // Create mutable reference to font database
    if let Some(fontdb) = Arc::get_mut(&mut options.usvg_opt.fontdb) {
        if !options.skip_system_fonts {
            fontdb.load_system_fonts();
        }

        if has_text_nodes {
            // Extract font options before passing to load_fonts
            load_fonts(
                fontdb,
                &options.font_files,
                &options.font_dirs,
                options.serif_family.unwrap(),
                options.sans_serif_family.unwrap(),
                options.cursive_family.unwrap(),
                options.fantasy_family.unwrap(),
                options.monospace_family.unwrap(),
            );
        }
    }

    let tree = {
        resvg::usvg::Tree::from_xmltree(&xml_tree, &options.usvg_opt)
            .map_err(|e| e.to_string())
    }?;
    let pixmap = render_svg(options.background, options.fit_to, &tree)?;
    pixmap.encode_png().map_err(|e| e.to_string())
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
    style_sheet = None,
    resources_dir = None,
    languages = vec![],
    font_size = 16,
    font_family = None,
    serif_family = None,
    sans_serif_family = None,
    cursive_family = None,
    fantasy_family = None,
    monospace_family = None,
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
    // Style Sheet
    style_sheet: Option<String>,
    // Resource Directory
    resources_dir: Option<String>,
    // Fonts
    languages: Option<Vec<String>>,
    font_size: Option<u32>,
    mut font_family: Option<String>,
    mut serif_family: Option<String>,
    mut sans_serif_family:Option<String>,
    mut cursive_family: Option<String>,
    mut fantasy_family: Option<String>,
    mut monospace_family:Option<String>,
    // Font files and directories
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
    
    let none_or_take = |item:Option<String>,otherwise:&str|{
        if item.is_none(){
            Some(otherwise.to_owned())
        }else{
            item
        }
    };

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        font_family = none_or_take(font_family, "Times New Roman");
        serif_family = none_or_take(serif_family, "Times New Roman");
        sans_serif_family = none_or_take(sans_serif_family, "Arial");
        cursive_family = none_or_take(cursive_family, "Comic Sans MS");
        fantasy_family = none_or_take(fantasy_family, "Impact");
        monospace_family = none_or_take(monospace_family, "Courier New");
    }
     
    #[cfg(target_os="linux")] 
    {
        font_family = none_or_take(font_family, "Liberation Serif");
        serif_family = none_or_take(serif_family, "Liberation Serif");
        sans_serif_family = none_or_take(sans_serif_family, "Liberation Sans");
        cursive_family = none_or_take(cursive_family, "Comic Neue");
        fantasy_family = none_or_take(fantasy_family, "Anton");
        monospace_family = none_or_take(monospace_family, "Liberation Mono");
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
        return Err(PyValueError::new_err("`svg_string` is empty or `svg_path` contains empty invalid svg"));
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
        Some(value) => Some(std::fs::canonicalize(value)?),
        None => None,
    };

    let _background = match background {
        Some(color_str) => match color_str.parse::<svgtypes::Color>() {
            Ok(color) => Some(color),
            Err(error) => panic!("Error background: {}", error),
        },
        None => None,
    };

    let fontdb = resvg::usvg::fontdb::Database::new();
    
    let usvg_options = resvg::usvg::Options {
        resources_dir: _resources_dir,
        dpi: dpi.unwrap_or(0) as f32,
        font_family: font_family.unwrap(),
        font_size: font_size.unwrap_or(16) as f32,
        languages: languages.unwrap_or(vec![]),
        shape_rendering: _shape_rendering,
        text_rendering: _text_rendering,
        image_rendering: _image_rendering,
        style_sheet: Some(style_sheet.unwrap_or_default()),
        default_size,
        image_href_resolver: resvg::usvg::ImageHrefResolver::default(),
        fontdb: Arc::new(fontdb),
        font_resolver: FontResolver::default(),
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
    match resvg_magic(options, _svg_string.trim().to_owned()) {
        Ok(pixmap) => Ok(pixmap),
        Err(e) => Err(PyValueError::new_err(e)),
    }
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
    m.add("__version__", get_version())?;
    m.add("__author__", get_author())?;
    m.add_function(wrap_pyfunction!(svg_to_bytes, m)?)?;
    Ok(())
}
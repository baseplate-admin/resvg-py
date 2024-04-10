/*
Based on
* https://github.com/RazrFalcon/resvg/blob/master/crates/resvg/src/main.rs
* https://github.com/mrdotb/resvg_nif/blob/master/native/resvg/src/lib.rs
*/

use base64::{engine::general_purpose, Engine as _};
use pyo3::prelude::*;
use resvg;
use resvg::usvg;

struct Opts {
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

fn load_font(options: &mut Opts, fontdb: &mut usvg::fontdb::Database) {
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

fn render_svg(tree: &usvg::Tree) -> Result<tiny_skia::Pixmap, String> {
    let mut pixmap = tiny_skia::Pixmap::new(
        tree.size().to_int_size().width(),
        tree.size().to_int_size().height(),
    )
    .unwrap();
    let ts = tree.view_box().to_transform(tree.size());
    resvg::render(tree, ts, &mut pixmap.as_mut());

    Ok(pixmap)
}

fn resvg_magic(svg_string: String) -> Result<Vec<u8>, String> {
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

    let tree = {
        usvg::Tree::from_xmltree(&xml_tree, &usvg::Options::default(), &fontdb)
            .map_err(|e| e.to_string())
    }
    .unwrap();
    let img: Vec<u8> = render_svg(&tree).unwrap().encode_png().unwrap();
    Ok(img)
}

#[pyfunction]
fn svg_to_base64(svg_string: String) -> PyResult<String> {
    let pixmap = resvg_magic(svg_string).unwrap();
    Ok(general_purpose::STANDARD.encode(&pixmap))
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(svg_to_base64, m)?)?;
    Ok(())
}

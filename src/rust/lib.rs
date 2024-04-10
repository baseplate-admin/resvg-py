use base64::{engine::general_purpose, Engine as _};
use pyo3::prelude::*;
use resvg;
use resvg::usvg;
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

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(py: Python, svg_string: String) -> PyResult<String> {
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
    Ok(general_purpose::STANDARD.encode(&img))
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

use pyo3::prelude::*;
use resvg::usvg;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(svg_string: String) -> PyResult<String> {
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
    };

    Ok(("Hello").to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn resvg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

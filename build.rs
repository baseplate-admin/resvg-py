use std::fs;

fn main() {
    let lock = fs::read_to_string("Cargo.lock").unwrap();

    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        if line.trim() == "name = \"resvg\"" {
            if let Some(version_line) = lines.find(|l| l.trim().starts_with("version")) {
                let version = version_line
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .trim_matches('"');

                println!("cargo:rustc-env=DEP_RESVG_VERSION={version}");
                break;
            }
        }
    }
}

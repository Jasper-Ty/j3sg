pub mod page;
pub mod section;

use std::fs;
use std::path::Path;
use tera::Tera;
use crate::section::Section;

pub fn generate(src_dir: &Path, out_dir: &Path, template_dir: &Path) -> std::io::Result<()> {
    if !out_dir.is_dir() {
        fs::create_dir(out_dir)?;
    }
    let glob = template_dir.join("**/*.html").to_string_lossy().to_string();
    let mut tera = Tera::new(&glob).unwrap();
    tera.autoescape_on(vec![]);

    let sections = Section::get_dir_sections(src_dir, out_dir)?;
    for section in sections {
        section.render(&tera)?;
    }

    Ok(())
}

pub fn init(src_dir: &Path, out_dir: &Path, template_dir: &Path, static_dir: &Path) -> std::io::Result<()> {

    fs::create_dir_all(src_dir)?;
    fs::create_dir_all(out_dir)?;
    fs::create_dir_all(template_dir)?;
    fs::create_dir_all(static_dir)?;

    Ok(())
}

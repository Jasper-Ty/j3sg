//! Wassup homies this is j3sg-gen
//!
//!
//!
//!
//!
//!

mod page;
mod section;
mod uri;
mod filesystem;
mod parse;
mod render;
mod sitemap;
mod init;

use std::fs::{self, OpenOptions};
use std::path::Path;
use tera::{Tera, Context};

use sitemap::SiteMap;
use page::PageMap;
use section::SectionMap;


/// Generates the site
///
/// TODO: Break this function TF up
pub fn generate<B, T, S>(
    src_dir: B, 
    out_dir: T, 
    template_dir: S
) -> Result<(), String> 
where
    B: AsRef<Path>,
    T: AsRef<Path>,
    S: AsRef<Path>
{
    let (src_dir, out_dir, template_dir) = (src_dir.as_ref(), out_dir.as_ref(), template_dir.as_ref());

    if !out_dir.is_dir() {
        fs::create_dir(out_dir)
            .map_err(|e| e.to_string())?;
    }

    // 01 -- Parse source directory structure
    let sitemap = SiteMap::new(src_dir)?;
    println!("The generated site will have the following structure: ");
    sitemap.print_tree();


    // 02 -- Read in source files
    let sectionmap = SectionMap::new(&sitemap)?;
    let pagemap = PageMap::new(&sitemap)?;
    // TODO: Check that root index is there and has a nonempty template field!
    let root_index = src_dir.join("index.md");
    if !root_index.is_file() {
        return Err("No root index file found".to_string());
    }
    // TODO: Fill in empty fields
    

    // 03 -- Render
    // insert global objects 
    let mut context = Context::new();
    context.insert("SECTION_MAP", &sectionmap);
    context.insert("PAGE_MAP", &pagemap);

    // TODO: move this tera init to a function
    let glob = template_dir.join("**/*.html").to_string_lossy().to_string();
    let mut tera = Tera::new(&glob)
        .map_err(|e| e.to_string())?;
    tera.autoescape_on(vec![]);

    // finally render every page
    for (uri, page) in pagemap.0.iter() {
        fs::create_dir_all(uri.out_dir(out_dir))
            .map_err(|_| "unable to create folder".to_string())?;

        let section = sectionmap.0.get(&page.section)
            .ok_or("Page with no section?".to_string())?;
    
        context.insert("page", page);
        context.insert("section", section);

        let out_path = uri.out_path(out_dir);
        let outfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&out_path)
            .map_err(|e| format!("Error writing output file: {}", e))?;
        let template = "base.html";
        tera.render_to(template, &context, outfile)
            .map_err(|e| e.to_string())?;

    }

    let sectioniter = sectionmap.0.iter()
        .filter(|(_, section)| section.index.is_some());
    for (uri, section) in sectioniter {
        fs::create_dir_all(uri.out_dir(out_dir))
            .map_err(|_| "unable to create folder".to_string())?;

        context.insert("page", &section.index);
        context.insert("section", section);

        let out_path = uri.out_path(out_dir);
        let outfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&out_path)
            .map_err(|e| format!("Error writing output file: {}", e))?;
        let template = "base.html";
        tera.render_to(template, &context, outfile)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn init(src_dir: &Path, out_dir: &Path, template_dir: &Path, static_dir: &Path) -> std::io::Result<()> {

    fs::create_dir_all(src_dir)?;
    fs::create_dir_all(out_dir)?;
    fs::create_dir_all(template_dir)?;
    fs::create_dir_all(static_dir)?;

    filesystem::cat(src_dir.join("index.md"), init::ROOT_INDEX)?;
    filesystem::cat(template_dir.join("base.html"), init::BASE_TEMPLATE)?;
    Ok(())
}


#[cfg(test)]
pub mod test {
    use super::*;
}

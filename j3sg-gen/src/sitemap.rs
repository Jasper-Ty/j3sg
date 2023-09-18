//! A struct which holds the generated site's structure
//!

use crate::filesystem::{file_name, file_stem, files_with_extension, has_file, subdirs};
use crate::uri::Uri;
use serde::Serialize;
use colored::*;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub struct SiteMap {
    pub sections: HashMap<Uri, PathBuf>,
    pub subsections: HashMap<Uri, Vec<Uri>>,
    pub pages: HashMap<Uri, PathBuf>, 
    pub subpages: HashMap<Uri, Vec<Uri>>, 
}
impl SiteMap {
    pub fn new<P>(src_dir: P) -> Result<Self, String> 
    where 
        P: AsRef<Path>
    {
        let src_dir = src_dir.as_ref();
        let mut sitemap = Self {
            sections: HashMap::new(),
            subsections: HashMap::new(),
            pages: HashMap::new(),
            subpages: HashMap::new(),
        };
        sitemap.sections.insert(Uri::new(), src_dir.to_owned());
        sitemap.build(&Uri::new())?;

        Ok(sitemap)
    }

    /// Recursively traverses the subdirectories of section[uri]  
    /// and inserts the corresponding subsections and subpages
    fn build(&mut self, uri: &Uri) -> Result<(), String> {
        let dir = self.sections.get(uri)
            .ok_or("Uri does not exist in PageMap".to_string())?;

        // Traverses subdirectories to find immediate subsections and subpages
        let (sections, pages) = {
            let mut pages: Vec<PathBuf> = files_with_extension(&dir, "md")?
                .filter(|path| file_name(path).unwrap() != "index.md").collect();
            let mut sections: Vec<PathBuf> = Vec::new();
            let mut stack: Vec<PathBuf> = subdirs(&dir)?.collect();
            while let Some(subdir) = stack.pop() {
                if has_file(&subdir, "index.md")? {
                    sections.push(subdir);
                } else {
                    stack.extend(subdirs(&subdir)?);
                    pages.extend(files_with_extension(&subdir, "md")?);
                }
            }
            (sections, pages)
        };
        
        let mut subsections = Vec::new();
        for section in sections {
            let section_uri = uri.join(file_name(&section)?)?;
            subsections.push(section_uri.clone());
            self.sections.insert(section_uri.clone(), section);
            
            // Recursive call
            self.build(&section_uri)?;
        }
        self.subsections.insert(uri.clone(), subsections);

        let mut subpages = Vec::new();
        for page in pages {
            let page_uri = uri.join(file_stem(&page)?)?;
            subpages.push(page_uri.clone());
            self.pages.insert(page_uri, page);
        }
        self.subpages.insert(uri.clone(), subpages);
        Ok(())
    }

    fn draw_uri_tree(&self, uri: &Uri, level: u32) {
        let subsections = self.subsections.get(uri).unwrap();
        let subpages = self.subpages.get(uri).unwrap();

        for _ in 0..level {
            print!("  ");
        }
        println!("{}/", uri.file_name().blue());
        for _ in 0..level+1 {
            print!("  ");
        }
        println!("({})", "index".red());

        for subsection_uri in subsections {
            self.draw_uri_tree(subsection_uri, level+1);
        }

        for i in 0..subpages.len() {
            let page_uri = subpages.get(i).unwrap();

            for _ in 0..level+1 {
                print!("  ");
            }
            let file_name = page_uri.file_name();
            if file_name == "index" {
                println!("{}", page_uri.file_name().red());
            } else {
                println!("{}", page_uri.file_name().green());
            }
        }
    }

    pub fn print_tree(&self) {
        self.draw_uri_tree(&Uri::new(), 0);
    }
}

#[derive(Debug, Serialize)]
pub struct SiteNode {
    pub uri: Uri,
    pub src: PathBuf,
}
impl SiteNode {
    pub fn new<P>(src: P) -> Self 
    where
        P: AsRef<Path>
    {
        let uri = Uri::new();
        let src = src.as_ref().to_owned();
        Self {
            uri,
            src,
        }
    }

    fn new_child<P>(&self, src: P) -> Result<Self, String>
    where
        P: AsRef<Path>
    {
        let uri = self.uri.join(file_name(&src)?)?;
        let src = src.as_ref().to_owned();
        Ok(Self {
            uri,
            src,
        })
    }

    pub fn pages(&self) -> Result<Vec<Self>, String> {
        let src = &self.src;
        let mut pages: Vec<SiteNode> = files_with_extension(src, "md")?
            .filter_map(|path| self.new_child(path).ok())
            .collect();

        let mut stack: Vec<PathBuf> = subdirs(src)?.collect();
        while let Some(subdir) = stack.pop() {
            if !has_file(&subdir, "index.md")? {
                stack.extend(subdirs(&subdir)?);
                pages.extend(files_with_extension(&subdir, "md")?
                             .filter_map(|path| self.new_child(path).ok()));
            }
        }
        Ok(pages)
    }

    pub fn subsections(&self) -> Result<Vec<Self>, String> {
        let mut subsections = Vec::new();
        let mut stack: Vec<PathBuf> = subdirs(&self.src)?.collect();

        while let Some(subdir) = stack.pop() {
            if has_file(&subdir, "index.md")? {
                let uri = self.uri.join(file_name(&subdir)?)?;
                let src = subdir.to_owned();
                subsections.push(Self {
                    uri, 
                    src,
                })
            } else {
                stack.extend(subdirs(subdir)?);
            }
        }
            
        Ok(subsections)
    }
}

#[cfg(test)]
pub mod test {
}

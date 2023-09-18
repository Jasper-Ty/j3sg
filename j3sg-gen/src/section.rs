use crate::uri::Uri;
use crate::sitemap::SiteMap;
use crate::filesystem::file_name;
use crate::parse::Parse;
use crate::page::Page;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Serialize;
use serde_yaml::Value;

#[derive(Debug, Serialize)]
pub struct Section {
    pub uri: Uri,
    pub title: String,
    pub index: Option<Page>,
    pub subsections: Vec<Uri>,
    pub pages: Vec<Uri>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>, 
}
impl Section {
    pub fn new<P>(sitemap: &SiteMap, uri: Uri, src: P) -> Result<Self, String> 
    where
        P: AsRef<Path>
    {
        let src = src.as_ref();
        let subsections = sitemap.subsections.get(&uri)
            .ok_or("No subsection vec found?".to_string())?
            .clone();
        let pages = sitemap.subpages.get(&uri)
            .ok_or("No subpages vec found?".to_string())?
            .clone();

        let text = fs::read_to_string(src.join("index.md"))
            .map_err(|e| e.to_string())?;
        let parse = Parse::from_str(&text)?;
        let index = Page::new(
            uri.clone(),
            uri.clone(),
            src.join("index.md"),
        ).ok();

        let section = Self {
            uri,
            title: parse.title.unwrap_or(file_name(src)?),
            index,
            subsections,
            pages, 

            extra: parse.extra,
        };
        Ok(section)
    }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct SectionMap(pub HashMap<Uri, Section>);
impl SectionMap {
    pub fn new(sitemap: &SiteMap) -> Result<Self, String> {
        let mut map = HashMap::new();
        let sections = &sitemap.sections;
        for (uri, src) in sections {
            let section = Section::new(&sitemap, uri.clone(), src)?;
            println!("{:?}\n  {:?}\n  {:?}", section.uri, section.subsections, section.pages);
            map.insert(uri.clone(), section);
        }
        Ok(Self(map))
    }
}


#[cfg(test)]
pub mod test {
    use super::*;
}

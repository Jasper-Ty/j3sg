use crate::uri::Uri;
use crate::filesystem::file_stem;
use crate::parse::Parse;
use crate::sitemap::SiteMap;

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::Serialize;
use serde_yaml::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub uri: Uri,
    pub section: Uri,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub content: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>, 
}
impl Page {
    pub fn new<P>(uri: Uri, section_uri: Uri, src: P) -> Result<Self, String> 
    where
        P: AsRef<Path>
    {
        let src = src.as_ref();
        let text = fs::read_to_string(src)
            .map_err(|e| e.to_string())?;
        let parse = Parse::from_str(&text)?;
        let page = Page {
            uri,
            section: section_uri,
            title: parse.title.unwrap_or(file_stem(src)?),
            author: parse.author,
            description: parse.description, 
            template: parse.template,
            content: parse.content,

            extra: parse.extra,
        };
        Ok(page)
    }
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct PageMap(pub HashMap<Uri, Page>);
impl PageMap {
    pub fn new(sitemap: &SiteMap) -> Result<Self, String> {
        let mut map = HashMap::new();
        let subpages = &sitemap.subpages;
        for (section_uri, page_uris) in subpages {
            for page_uri in page_uris {
                let src = sitemap.pages.get(page_uri)
                    .ok_or("Page not found in sitemap")?;
                let page = Page::new(
                    page_uri.clone(),
                    section_uri.clone(),
                    src)?;
                map.insert(page_uri.clone(), page);
            }
        }
        Ok(Self(map))
    }
}

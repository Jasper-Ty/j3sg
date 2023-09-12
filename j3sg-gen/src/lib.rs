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

pub mod page {
    use std::fs;
    use std::path::PathBuf;
    use std::path::Path;
    use serde::{Serialize, Deserialize};
    use markdown::{
        mdast::{ Node, Root, Yaml },
        Constructs, Options, ParseOptions, CompileOptions
    };

    pub fn uri_from_src(src: &PathBuf, src_dir: &Path) -> String {
        let file_stem = src.file_stem().unwrap();
        let is_index = file_stem.to_string_lossy() == "index";

        let base = Path::new("/").join(src.strip_prefix(src_dir).unwrap())
            .parent().unwrap().to_owned();

        let uri = if is_index {
            base
        } else {
            base.join(file_stem)
        };

        uri.into_os_string().into_string().unwrap()
    }

    fn out_from_uri(uri: &str, out_dir: &Path) -> String {
        out_dir.join(
                Path::new(uri)
                .strip_prefix("/").unwrap())
            .join("index.html")
            .to_str().unwrap()
            .to_owned()
    }


    /// Struct containing front matter fields -- used in deserializing the front matter in each .md
    /// file.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FrontMatter {
        pub title: String,
        pub template: String,
        pub date: u64,
    }

    /// Container for all info about a single page.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Page {
        /// The path of the source .md file for this page
        pub src: String,
        /// The page's unique URI 
        pub uri: String,
        /// The path of the output .html file for this page
        pub out: String,
        /// The page title
        pub title: String,
        /// The template which renders this page
        pub template: String,
        /// The date associated with a page, as a UNIX Timestamp
        pub date: u64,
        /// The rendered html content of the page
        pub content: String,
        /// Whether or not this page is an index page
        pub index: bool,
    }
    impl Page {
        pub fn new(src: PathBuf, src_dir: &Path, out_dir: &Path) -> Self {
            let file_stem = src.file_stem().unwrap();
            let index = file_stem.to_string_lossy() == "index";

            // Generate page URI and output path
            let uri = uri_from_src(&src, src_dir);
            let out = out_from_uri(&uri, out_dir);

            // markdown-rs options
            let mdopts = Options {
                parse: ParseOptions {
                    constructs: Constructs {
                        frontmatter: true,
                        html_flow: true,
                        html_text: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                compile: CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            // Get page content
            let text = fs::read_to_string(&src).unwrap();
            let content = match markdown::to_html_with_options(&text, &mdopts) {
                Ok(s) => s,
                Err(_) => "Unable to render markdown!".to_string(),
            };

            let FrontMatter {
                title,
                template,
                date,
            } = parse_frontmatter(&text);

            Self {
                src: src.to_string_lossy().into_owned(),
                uri,
                out,
                content,
                title,
                template,
                date,
                index,
            }
        }
    }

    fn parse_frontmatter(text: &str) -> FrontMatter {
        let parseopts = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                html_flow: true,
                html_text: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // Get page front matter
        let Node::Root(Root { children, .. }) = markdown::to_mdast(text, &parseopts).unwrap() else {
            panic!("Cannot parse mdast")
        }; 

        children.get(0)
            .map(|node| match node {
                Node::Yaml(Yaml { value, .. }) => Some(value),
                _ => None
            })
            .flatten()
            .map(|s| serde_yaml::from_str::<FrontMatter>(s).ok())
            .flatten()
            .unwrap_or(FrontMatter {
                title: "NO TITLE".to_string(),
                template: "base-1.html".to_string(),
                date: 0, // now,
            })

    }
}

pub mod section {
    use crate::page::Page;

    use std::ffi::OsStr;
    use std::fs;
    use std::path::{Path, PathBuf};
    use serde::{Serialize, Deserialize};
    use tera::{Tera, Context};
    use log::info;

    /// A section is a group of pages in the same folder
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Section {
        pub title: String,
        pub pages: Vec<Page>
    }
    impl Section {

        pub fn new(title: String, dir: Vec<PathBuf>, src_dir: &Path, out_dir: &Path) -> Self {
            let pages = dir.into_iter()
                .map(|src| Page::new(src, src_dir, out_dir))
                .collect();
            Self {
                title,
                pages
            }
        }

        pub fn render(self, tera: &Tera) -> std::io::Result<()> {
            let mut context = Context::new();
            context.insert("section", &self);

            for page in self.pages {
                context.insert("page", &page);

                info!("Rendering {:?}", &page.uri);

                info!("-- creating directories...");
                fs::create_dir_all(Path::new(&page.out).parent().unwrap())?;

                info!("-- rendering template...");
                let output = tera.render(&page.template, &context).unwrap();

                info!("-- writing to {:?}...", &page.out);
                fs::write(&page.out, output)?;

                info!("-- done!");
            }

            Ok(())
        }

        pub fn get_dir_sections(src_dir: &Path, out_dir: &Path) -> std::io::Result<Vec<Self>> {

            let mut sections: Vec<Section> = vec![];
            let mut stack: Vec<PathBuf> = vec![src_dir.to_owned()];

            while let Some(dir) = stack.pop() {
                let title = dir.to_string_lossy().to_string();
                let subdirs = fs::read_dir(&dir)?
                    .filter_map(|entry| entry.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.is_dir());
                stack.extend(subdirs);

                let pages = fs::read_dir(&dir)? 
                    .filter_map(|entry| entry.ok()) 
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file())
                    .filter(|path| path
                                .extension()
                                .map(OsStr::to_str)
                                .flatten() == Some("md"))
                    .map(|path| Page::new(path, src_dir, out_dir))
                    .collect();

                sections.push(Section{
                    title,
                    pages
                })
            }

            Ok(sections)
        }
    }
}

use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml::Value;
use markdown::{
    mdast::{Node, Root, Yaml},
    to_html_with_options, to_mdast,
    Constructs, Options, ParseOptions
};

#[derive(Debug)]
pub struct Parse {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub content: String,

    pub extra: HashMap<String, Value>, 
}
impl Parse {
    pub fn from_str(text: &str) -> Result<Self, String> {
        let opts = Options {
            parse: ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    html_flow: true,
                    html_text: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let frontmatter = FrontMatter::from_str(text)
            .unwrap_or(Default::default());
        let content = to_html_with_options(text, &opts)?;

        Ok(Self {
            title: frontmatter.title,
            author: frontmatter.author,
            description: frontmatter.description,
            template: frontmatter.template,
            extra: frontmatter.extra,
            content,
        })
    }
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    template: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>, 
}
impl Default for FrontMatter {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            description: None,
            template: None,
            extra: HashMap::new(),
        }
    }
}
impl FrontMatter {
    fn from_str(text: &str) -> Option<Self> {
        let parseopts = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let ast = to_mdast(text, &parseopts).ok()?;

        let Node::Root(Root { children, .. }) = ast 
            else { return None };
        
        let Some(Node::Yaml(Yaml {
            value, ..
        })) = children.get(0) 
            else { return None };

        serde_yaml::from_str(value).ok()
    }
}

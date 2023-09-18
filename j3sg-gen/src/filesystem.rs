//! Helper filesystem functions

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs;

/// Returns an iterator of the subdirectories of `dir`
pub fn subdirs<P: AsRef<Path>>(dir: P) -> Result<impl Iterator<Item=PathBuf>, String> {
    Ok(fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir()))
}

/// Returns the file name of a path
pub fn file_name<P: AsRef<Path>>(path: P) -> Result<String, String> {
    path
       .as_ref()
       .file_name()
       .map(OsStr::to_str)
       .flatten()
       .map(String::from)
       .ok_or("Unable to peek".to_string())
}

/// Returns the file stem of a path
pub fn file_stem<P: AsRef<Path>>(path: P) -> Result<String, String> {
    path
       .as_ref()
       .file_stem()
       .map(OsStr::to_str)
       .flatten()
       .map(String::from)
       .ok_or("Unable to peek".to_string())
}

// ugly ass fucking function signature
pub fn files<P: AsRef<Path>>(dir: P) -> Result<impl Iterator<Item=PathBuf>, String> {
    Ok(fs::read_dir(dir.as_ref()) 
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file()))
}

pub fn has_file<P: AsRef<Path>>(dir: P, filename: &str) -> Result<bool, String> {
    Ok(files(dir)?.any(|file|
        file.file_name()
            .map(|osstr| osstr.to_str())
            .flatten()
        == Some(filename)
    ))
}

pub fn files_with_extension<P: AsRef<Path>>(dir: P, ext: &'static str) -> Result<impl Iterator<Item=PathBuf>, String> {
    files(dir)
        .map(|iter| iter
             .filter(|path| path.extension()
                     .map(|s| s.to_owned().to_str() == Some(ext)) == Some(true))
        )
}

fn touch<P>(path: P) -> std::io::Result<()> 
where 
    P: AsRef<Path>
{
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    Ok(())
}

pub fn cat<P>(path: P, text: &[u8]) -> std::io::Result<()>
where
    P: AsRef<Path>
{
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?
        .write_all(text)
}

pub const TEST_DIR: &str = "/tmp/j3sg-test";
pub const TEST_SRC: &str = "/tmp/j3sg-test/src";
pub const TEST_OUT: &str = "/tmp/j3sg-test/out";
pub const TEST_TEMPLATES: &str = "/tmp/j3sg-test/templates";
pub fn init_test_dir() -> std::io::Result<()> {
    fs::remove_dir_all(TEST_DIR).unwrap_or(());

    fs::create_dir_all(TEST_DIR)?;
    fs::create_dir_all(TEST_SRC)?;
    fs::create_dir_all(TEST_OUT)?;
    fs::create_dir_all(TEST_TEMPLATES)?;

    std::env::set_current_dir(TEST_SRC)?;

    cat("index.md", 
        b"---\ntitle: Index\ntemplate: base.html\n---\n # Hello")?;
    cat("page1.md", 
        b"---\ntitle: Page 1\ntemplate: base.html\n---\n# This is the first page\nWelcome to the first page of this site.")?;
    touch("page2.md")?;
    touch("page3.md")?;

    fs::create_dir_all("foo/bar/baz")?;
    touch("foo/bar/baz/index.md")?;
    touch("foo/bar/baz/foobarbaz.md")?;

    fs::create_dir("nest1")?;
    touch("nest1/index.md")?;
    fs::create_dir("nest1/nest2")?;
    touch("nest1/nest2/index.md")?;

    fs::create_dir("blog")?;
    touch("blog/index.md")?;
    fs::create_dir("blog/2021")?;
    touch("blog/2021/2021-post.md")?;
    fs::create_dir("blog/2022")?;
    touch("blog/2022/2022-post.md")?;
    fs::create_dir("blog/2023")?;
    touch("blog/2023/2023-post.md")?;

    fs::create_dir("tools")?;
    fs::create_dir("tools/pickaxe")?;
    touch("tools/pickaxe/index.md")?;
    fs::create_dir("tools/shovel")?;
    touch("tools/shovel/index.md")?;
    fs::create_dir("tools/hoe")?;
    touch("tools/hoe/index.md")?;

    fs::create_dir("no_index")?;
    touch("no_index/page4.md")?;
    touch("no_index/page5.md")?;

    fs::create_dir("empty")?;

    std::env::set_current_dir(TEST_TEMPLATES)?;
    cat("base.html", 
b"<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <title>{{ page.title }}</title>
    </head>

    <body>
        <nav>
            {{ section.title }}
            {% for uri in section.subsections %}
                {% set subsection = SECTION_MAP[uri] %}
                <a href=\"{{ uri }}\">{{ subsection.title }}</a>
            {% endfor %}
        </nav> 
        <main>
            {{ page.content }}
        </main>
    </body>
</html>"
    )?;
    Ok(())
}

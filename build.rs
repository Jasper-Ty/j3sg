use std::collections::HashMap;
use std::fs;
use std::path::Path;


/// Takes a file with KEY="VALUE" pairs and returns a
/// HashMap created from it
pub fn read_page(page_path: &str) -> HashMap<Vec<u8>, Vec<u8>> {
    let mut map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

    let f = fs::read(page_path)
        .expect("Should be able to read the file");
    let mut s = &f[..];

    'kv: loop {
        let mut key: Vec<u8> = Vec::new();
        'k: loop {
            match s {
                [b' ' | b'\n', rest @ ..] => { s = rest; },
                [b'=', rest @ ..] => { s = rest; break 'k;}
                [c, rest @ ..] => { key.push(*c); s = rest; },
                [] => break 'kv,
            }
        }
        's: loop {
            match s {
                [b' ' | b'\n', rest @ ..] => { s = rest; },
                [b'"', rest @ ..] => { s = rest; break 's },
                [..] => panic!("Unquoted value"),
            }
        }
        let mut value: Vec<u8> = Vec::new();
        'v: loop {
            match s {
                [b'"', rest @ ..] => { s = rest; break 'v; },
                [b'\\', b'"', rest @ ..] => { value.push(b'"'); s = rest; }
                [c, rest @ ..] => { value.push(*c); s = rest; },
                [] => panic!("Unexpected EOF"),
            }
        }
        map.insert(key, value);
    }

    map
}


/// Takes a path to a template and a path to a page 
/// and compiles the template with page data, 
/// subtituting {{ KEY }} with VALUE
pub fn render_page(page_path: &str, template_path: &str) -> String {
    let data = read_page(page_path);
    let template = fs::read(template_path)
        .expect("Should be able to read the file");

    let mut out: Vec<u8> = Vec::with_capacity(template.len());
    let mut id: Vec<u8> = Vec::new();
    let mut s = &template[..];
    'parsing: loop {
        match s {
            [b'\\', c @ b'{', rest @ ..]             
            | [b'\\', c @ b'}', rest @ ..] => { out.push(*c); s = rest; },
            [b'{', b'{', rest @ ..] => {
                s = rest;
                'b: loop {
                    match s {
                        [b' ', rest @ ..] => { s = rest; },
                        [b'}', b'}', rest @ ..] => { s = rest; break 'b; },
                        [c, rest @ ..] => { id.push(*c); s = rest; },
                        [] => break 'b, 
                    }
                }
                if let Some(v) = data.get(&id) {
                    out.append(&mut v.to_owned());
                }
                id.clear();
            },
            [c, rest @ ..] => { out.push(*c); s = rest; },
            [] => break 'parsing
        }
    }

    String::from_utf8(out)
        .expect("Should be valid UTF-8")
}

fn main() {
    let page = render_page("./pages_src/index", "./templates/simple.html");
    let path = Path::new("./pages").join("index.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/bio", "./templates/simple.html");
    let path = Path::new("./pages").join("bio.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/projects", "./templates/simple.html");
    let path = Path::new("./pages").join("projects.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/art", "./templates/simple.html");
    let path = Path::new("./pages").join("art.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/notes", "./templates/simple.html");
    let path = Path::new("./pages").join("notes.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/blog", "./templates/simple.html");
    let path = Path::new("./pages").join("blog.html");
    fs::write(&path, page).unwrap();

    let page = render_page("./pages_src/misc", "./templates/simple.html");
    let path = Path::new("./pages").join("misc.html");
    fs::write(&path, page).unwrap();
}

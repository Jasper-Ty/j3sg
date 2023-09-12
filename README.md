# j3sg

## Information

j3sg stands for *Jasper's Simple Static Site Generator*. This is still in a very early stage. There is little in the way of customization or error handling at the moment.

## Usage

``` 
$ j3sg COMMAND 
```

The available commands are

```
COMMANDS:
    "gen" | "generate" | "G"
        Compiles the source files in ./src 
    "srv" | "serve" | "S"
        Starts the static file server
```

### Directory Structure

j3sg requires this directory structure

```
.
├── templates 
├── src
├── static
└── public 
```

 - `templates` should contain Tera templates
 - `src` is where your Markdown files live
 - Files in `static` are served at `/static/**/*`
 - Generated HTML files and directories go in `public`, and is served at `/**/*`

### Generate

j3sg renders pages as Markdown files to HTML using [markdown-rs](https://github.com/wooorm/markdown-rs). The structure of the output folder and the page URIs depends on the file structure of `src`, specifically:

Each generated HTML file is a `index.html` file with directories generated to match the original filename, in order to best work with most servers. E.g

```
src/foo/bar.md -> public/foo/bar/index.html
```

so that the page is accessible at `/foo/bar`

The exception is with `index.md` files themselves, which do not get an extra directory. 

```
src/foo/index.md -> public/foo/index.html
```

(CURRENTLY NO PROTOCOL IN PLACE IF EITHER OF THEM CLASH, ONE WILL OVERWRITE THE OTHER DEPENDING ON HOW THE SRC DIRECTORY IS TRAVERSED)


#### Pages and Templates

j3sg uses [Tera](https://keats.github.io/tera/) for templating. It's very similar to Jinja.

In the context, j3sg exposes a `page` object, containing the following fields

```
src: The path to the source .md file
out: The path to the rendered .html file
uri: The page's URI
title: The page's title
template: The template file used to render the page
date: (DOES NOT WORK YET) 
content: The content of the .md file, rendered to HTML 
```

To set the title, template, and date, each Markdown file must have a *front matter*, which is a YAML block delimited by two fences as follows

```
---
title: Index
template: base.html
date: (DOES NOT WORK YET)
---

# Index

... rest of markdown ...
```

### Serve

It's a static file server with [Actix](https://actix.rs/). It's not very exciting right now.

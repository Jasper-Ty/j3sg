//! Init defaults

pub const ROOT_INDEX: &[u8] = 
b"---
title: My Site
author: Alice
description: A brand new site
template: base.html
---

# My Site

## Welcome

Congratulations! You've succesfully initialized this site and,
if you're viewing this in the browser, successfully built it.

## Next Steps

Make more pages, make more templates
"
;

pub const BASE_TEMPLATE: &[u8] = 
b"<html lang=\"en\">
    <head>
        <title>{{ page.title }}</title>

        {% if page.author %}
        <meta name=\"author\" content=\"{{ page.author }}\">
        {% endif %}

        {% if page.description %}
        <meta name=\"description\" content=\"{{ page.description }}\">
        {% endif %}
    </head>

    <body>
        {{ page.content }}
    </body>
</html>"
;

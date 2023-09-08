---
title: Projects
template: base-1.html
---

# Projects

This is a non-exhaustive list of personal projects I've done.

## jqoiview

<div>
    <a class="imglink" href="/static/img/jqoiview.png" target="_blank" width="100%">
        <img src="/static/img/jqoiview.png" width="100%">
    </a>
</div>

Program written in Rust for viewing the highly efficient [QOI](https://qoiformat.org/) image format. Uses SDL2.

[Source](https://github.com/Jasper-Ty/jqoiview) [crates.io](https://crates.io/crates/jqoiview)

## dither\_{png,bmp}

<div style="display: flex;">
    <a class="imglink" href="/static/img/santorini.png" target="_blank" width="50%">
        <img src="/static/img/santorini.png" width="100%">
    </a>
    <a class="imglink" href="/static/img/santorini_dithered.png" target="_blank" width="50%">
        <img src="/static/img/santorini_dithered.png" width="100%">
    </a>
</div>
<div>
    <a class="imglink" href="/static/img/your_name_dithered.bmp" target="_blank" width="100%">
        <img src="/static/img/your_name_dithered.bmp" width="100%">
    </a>
</div>

Simple command-line programs, written in C (dither\_png) and Rust (dither\_bmp) that take in an input image and produces a dithered output image.

[Source (dither\_png)](https://github.com/Jasper-Ty/dither_png) [Source (dither\_bmp)](https://github.com/Jasper-Ty/dither_bmp)

## j3sg

Stands for *Jasper's Simple Static Site Generator*. This site runs on it.

[Source](https://github.com/Jasper-Ty/jty-website)

## rustsweeper

<div>
    <a class="imglink" href="/static/img/rustsweeper.png" target="_blank" width="100%">
        <img src="/static/img/rustsweeper.png" width="100%">
    </a>
</div>

Work-in-progress clone of Minesweeper written in Rust. At the moment, it's using SDL2, but I may move it to [Bevy](https://bevyengine.org/). Someday I hope it reaches feature parity with [Minesweeper Arbiter](https://minesweepergame.com/download/arbiter.php) (the currently standard Minesweeper clone)

[Source](https://github.com/Jasper-Ty/rustsweeper)

## wordle solver

<div>
    <a class="imglink" href="/static/img/wordlesolver.png" target="_blank" width="100%">
        <img src="/static/img/wordlesolver.png" width="100%">
    </a>
</div>

Back when wordle was a craze, I quickly cooked up a solver for it in Python, basing it off of how Donald Knuth proved Mastermind can be solved in at most 5 moves using a maxmin algorithm. (Mastermind is one of my favorite puzzles).

It is hilariously slow due to the naiveness of both approach and implementation. I wrote an optimized version in C at some point, but the code for that is lost to time.

[Source](/static/misc/wordle_cmdline_solver.py)

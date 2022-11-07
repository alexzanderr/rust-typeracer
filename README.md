
![logo](https://github.com/alexzanderr/rust-typeracer/blob/main/static/img/logo/rust-typeracer-logo.png?raw=True)


<p align="center">
    <a href="https://alexzanderr.github.io/rust-python-objects/book/index.html">
        <img src="https://github.com/alexzanderr/rust-python-objects/actions/workflows/deploy_book.yaml/badge.svg" alt="book_deploy">
    </a>
    <br>
    <a href="">
        <img src="https://github.com/alexzanderr/rust-python-objects/actions/workflows/vulnerabilities_audit.yaml/badge.svg" alt="vulnerabilities_audit">
    </a>
    <br>
    <a href="https://crates.io/crates/python-objects">
        <img src="https://github.com/alexzanderr/rust-python-objects/actions/workflows/crates_io_publish.yaml/badge.svg" alt="crates_io_publish">
    </a>
    <br>
    <a href="https://docs.rs/python-objects/latest/python/">
        <img src="https://img.shields.io/badge/docs.rs-python-objects">
    </a>
    <br>
    <a href="https://crates.io/crates/python-objects">
        <img src="https://img.shields.io/crates/v/python-objects.svg" alt="Crates.io">
    </a>
    <br>
    <a href="https://choosealicense.com/licenses/mit/" alt="License: MIT">
        <img src="https://img.shields.io/badge/license-MIT-green.svg" />
    </a>
    <br>
    <a href="">
        <img src="https://img.shields.io/github/stars/alexzanderr/rust-python-objects?style=social">
    </a>
    <a href="">
        <img src="https://img.shields.io/github/forks/alexzanderr/rust-python-objects?style=social">
    </a>
    <a href="">
        <img src="https://img.shields.io/github/watchers/alexzanderr/rust-python-objects?style=social">
    </a>
    <br>
    <a>
        <img src="https://img.shields.io/github/repo-size/alexzanderr/rust-python-objects.svg" alt="repo_size">
    </a>
    <br>
    <a href="">
        <img src="https://img.shields.io/github/last-commit/alexzanderr/rust-python-objects">
    </a>
    <a href="">
        <img src="https://img.shields.io/github/release-date/alexzanderr/rust-python-objects">
    </a>
    <br>
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/badge/rustc-1.60+-yellow?logo=rust">
    </a>
    <br>
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/crates/d/python-objects">
    </a>
    <br>
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/maintenance/yes/2022">
    </a>
    <br>
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/github/contributors/alexzanderr/rust-python-objects">
    </a>
    <br>
    <a href="https://www.rust-lang.org">
        <img src="https://img.shields.io/crates/l/python-objects.svg">
    </a>
</p>

# Rust Typeracer
Typeracer TUI game written in Rust to play in the terminal.


# Table of Contents
- [Table of Contents](#table-of-contents)
    - [Showcases](#showcases)
    - [Screenshots](#screenshots)
    - [Game Features](#game-features)
    - [Installation](#installation)
    - [Documentation](#documentation)
    - [Why?](#why)
    - [TODOs](#todo)
    - [Contributing](#contributing)
    - [Changelog](#changelog)
    - [Projects That Use Rust Typeracer](#projects-that-use-rust-typeracer)
    - [NOTE](#note)
    - [ISSUES](#issues)
    - [Extra](#extra)


# Showcases
the game in action

# Screenshots

# Game Features

# Installation

## typeracer as a rust library
### 1. the modern and simple way

just run this command
```shell
cargo add typeracer # or tty-racer 
```
and this will add the `latest version` from `crates.io` to your `Cargo.toml`, just like the old way, but automatically.

- what is `cargo add` ? -> its a cargo built-in sub command to add depdendencies to your project automatically; see more in the [`docs`](https://doc.rust-lang.org/cargo/commands/cargo-add.html)

### 2. the old way

just copy the `crate name` and the version you want to use:
```toml
[dependencies]
# ...
# ...
# ...
typeracer = "$the_version_you_want"
# example
typeracer = "0.0.6"
```

to your `Cargo.toml` and then write some code and build your project that uses `python-objects`.

## typeracer as a rust standalone binary; the compiled game

```shell
cargo install typeracer # or tty-racer 
```

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Documentation

`docs.rs` official website

- [`docs.rs/rust-typeracer`](https://docs.rs/rust-typeracer/latest)
  the custom made book
- [`Rust Typeracer`](https://alexzanderr.github.io/rust-python-objects/index.html)

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Why

- why?

because typeracer is a very cool game. this project is heavily insipired by the original web
game: [`https://play.typeracer.com/]`(https://play.typeracer.com/)

- why this project when there are already typeracer rust games on the internet?

because none of them support these features:
- multi-line
- programming source code with syntax highlithing
- background music

and because they are not really configurable by the user, for example:
- i dont want block cursor
- i want to change the colors
- i want to change the UI layout
- and many more settings

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Other Rust Typeracer Games

- [`https://gitlab.com/ttyperacer/terminal-typeracer`](https://gitlab.com/ttyperacer/terminal-typeracer)
  honestly, by far this is the most advanced typeracer out there.
  I've looked in the source, its really well designed, the game supports `chinese` and it does statistics.


- [`https://github.com/ukmrs/smokey`](https://github.com/ukmrs/smokey)
- [`https://github.com/Samyak2/toipe`](https://github.com/Samyak2/toipe)
- [`https://github.com/krawieck/typer-rs`](https://github.com/krawieck/typer-rs)

Note: if you find other typeracer games please create an issue to tell me about them to put them here. Thank you.

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# TODOs

check [`todo/README.md`](https://github.com/alexzanderr/rust-typeracer/blob/main/todo/README.md)


<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Contributing

check [`CONTRIBUTING.md`](https://github.com/alexzanderr/rust-typeracer/blob/main/CONTRIBUTING.md
)

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Changelog

check [`CHANGELOG.md`](https://github.com/alexzanderr/rust-typeracer/blob/main/changelog/CHANGELOG.md)

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Projects That Use Rust Typeracer

incoming, would be nice.

using typeracer as library in their game, maybe they are making a game that has this library as sub-game

- [ ]
- [ ]

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# NOTEs

note for the user and developer

if you find `issues` go ahead and make an
[`issue`](https://github.com/alexzanderr/rust-typeracer/issues/new)
or a
[`pull request`](https://github.com/alexzanderr/rust-typeracer/compare),
cant wait to take a look into them.

peace to you!

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# ISSUES

check [`ISSUES.md`](https://github.com/alexzanderr/rust-typeracer/blob/main/ISSUES.md
)

<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

# Extra

## Git Configuration

to apply the settings from `.gitconfig` of this repo, run this command:

```shell
cd rust-python-objects # or dont cd if you are already in
git config --local include.path ../.gitconfig
```

this needs to run manually because it can never be automated due to security issues, see
this [`stack overflow answer`](https://stackoverflow.com/a/18330114/12172291)

for git config see also [`git-config`](https://git-scm.com/docs/git-config#_includes)

## Badges for readme

https://dev.to/envoy_/150-badges-for-github-pnk


<div align="right">
<a href="#table-of-contents">Back to TOC ☝️</a>
</div>

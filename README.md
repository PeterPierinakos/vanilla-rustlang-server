# Vanilla-RustLang-Server (VRS) 🦀

VRS is a simple, minimal, free and open source static web server written in Rust which uses absolutely no dependencies and revolves around Rust's std::net built-in utility.

## General Information

### VRS features

- Extremely lightweight binary after compiling due to its no dependency nature
- Easy-to-understand documentation & manual
- Multithreading (WIP)
- Singlethreading
- CORS (methods only, origins soon)
- Dockerfiles (Debian & Alpine)
- Customizable HTTP responses (200, 400 & 404, 405, ...)
- Guarantee to compile on all platforms (recommended: Linux or BSD)
- Easy customizability via its configuration file
- Security headers out of the box (origin attacks, iframe attacks, clickjacking etc.)
- HTTP/1.1 and HTTP/2 standard protocol versions (during development, it's recommended to use HTTP/1.1 because that's the only one Postman HTTP client supports.)
- Basic systemd service
- Logging (singlethread only.)

### What is a static web server?

A static web server is a kind of web server which only serves static content. This includes HTML, CSS and JS. Static web servers do _not_ have support for doing back-end business logic or interacting with databases out of the box.

### Example use cases of VRS

- A portfolio which is mostly HTML & CSS and a little bit of JS.
- A small wikipedia.

### Example use cases VRS is NOT suitable for

- A blog site.
- A social media website.
- A bank system.
- SpaceX clone.

### License

VRS is licensed under the [MIT License](https://mit-license.org/).

### When to use VRS?

You may consider using this piece of software if you meet any of the following criteria:

- You are looking for a simple static web server.
- You do not like unnecessary bloat for simple stuff.
- You are trying to repurpose ancient hardware and you do not want to wait for long compilation times.
- You want to learn the basics of multithreading (spawning threads).

## Installation

Read the docs to find out how to install and configure VRS (docs/ folder.)

> The primary features of the server have been added so expect less activity. I will still be maintaining this project and I will still be doing occasional releases for things such as bug fixes, QoL improvements, small features or minor adjustments.

# Vanilla-RustLang-Server (VRS) 🦀

VRS is a simple, minimal, free and open source static web server written in Rust which uses absolutely no dependencies and revolves around Rust's `std::net` standard library module.

## General Information

### VRS features

- ~1k SLOC; minimal
- Standalone, small and statically-linked binary after compilation with all of its configuration done at compile-time
- Easy-to-understand usage and configuration instructions
- Multithreading
- Singlethreading
- CORS (methods only, origins soon)
- Dockerfiles (Debian & Alpine)
- Customizable HTTP responses (200, 400 & 404, 405...)
- Guarantee to compile on all platforms (recommended: Linux or BSD)
- Easy customizability via its [configuration file](src/configuration.rs)
- Security headers out of the box (origin attacks, iframe attacks, clickjacking etc)
- HTTP/1.1 and HTTP/2 standard protocol versions
- Basic systemd service
- Filesystem caching for already requested documents
- Logging (singlethread only)

### What is a static web server?

A static web server is a kind of web server which only serves static content. This includes HTML, CSS and JS. Static web servers do _not_ have support for doing back-end business logic or interacting with databases out of the box.

### Example use cases of VRS

- A portfolio which is mostly HTML & CSS and a little bit of JS
- A small wikipedia

### Example use cases VRS is NOT suitable for

- A blog site
- A social media website
- A bank system
- SpaceX clone

### License

VRS is licensed under either [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.

### When to use VRS?

You may consider using this piece of software if you meet any of the following criteria:

- You are looking for a simple, reliable static web server
- You do not like unnecessary bloat for simple stuff
- You are trying to repurpose ancient hardware to host a server and you do not want to wait for long compilation times
- You need any of the features mentioned in the "VRS features" section

## Installation

Installations instructions can be found [here](docs/installation.md).

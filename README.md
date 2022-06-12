# Vanilla RustLang Server (VRS)

VRS is a simple, minimal, free and open source static web server written in Rust which uses absolutely no dependencies.

## General Information

### VRS features

- Extremely lightweight binary after compiling due to its no dependency nature
- Easy-to-understand documentation & manual in the form of HTML pages (inside media/manual.html)
- Multithreading (WIP)
- Singlethreading
- CORS (methods only, origins soon)
- Dockerfiles (Debian & Alpine)
- Customizable HTTP responses (200, 400 & 404)
- Guarantee to compile on all platforms (recommended: Linux or BSD)
- Easy customizability via its configuration file
- HTTP/1.1 and HTTP/2 standard protocol versions (during development, it's recommended to use HTTP/1.1 because that's the only one Postman HTTP client supports.)
- Basic systemd service
- Logging (singlethread only.)

### What is a static web server?

A static web server is a kind of web server which only serves static content. This includes HTML, CSS and JS. Static web servers do _not_ have support for doing back-end business logic or interacting with databases out of the box.

### Is VRS production-ready?

I see no problem with using it for production. However, please be informed that you should **NOT** pass sensitive data through JS or interact with databases as it was not meant to be used for this purpose. Additionally, since it is licensed under a FOSS license I have no warranty for any damages and you are free to make issues if you encounter any bugs.

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

### From source

1. Clone the repository change your directory;

```
git clone https://github.com/PeterPierinakos/vanilla-rustlang-server
cd vanilla-rustlang-server
```

2. Start the compilation process;

```
make build
```

3. Copy the HTML and run the program:

```
# (Root may be required)
./setup.sh
```

#### Docker

```
sudo docker build -f production/docker/alpine/Dockerfile -t vrs .
```

#### Without Docker

```
# (Root may be required)
sudo ./setup.sh
sudo make run
```

### Precompiled binary

Todo

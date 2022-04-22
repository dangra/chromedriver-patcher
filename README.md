# Patch ChromeDriver

`patch-cd` patches chromedriver binary to well known DOM injections that
leak the browser is under scripted control.

It does so by loading chromedriver binary into memory, modifying some identifiers
in place respecting binary size, and fexecve'ing directly from
memory which doesn't require filesystem write permission of any kind.

For more information on relevant patches:
* https://stackoverflow.com/questions/33225947/
* https://dev.to/tonetheman/what-is-the-cdc-variable-in-chromedriver-553p

For patching and execing directly from memory:

* https://magisterquis.github.io/2018/03/31/in-memory-only-elf-execution.html
* https://man7.org/linux/man-pages/man2/memfd_create.2.html
* https://man7.org/linux/man-pages/man3/fexecve.3.html


## Development

`memfd_create` and `fexecve` syscalls are only available on Linux, so full
development environment requires a linux system (see bellow). But to easy development on
other operating systems, the same functionality was implemented using a named
temporal file and `execve` syscall.

### Building for linux with Docker 

To compile for Linux on MacOS:

	docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.53.0 cargo build --release


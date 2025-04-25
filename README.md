# ChromeDriver Patcher

`patch-cd` is a utility that modifies ChromeDriver binaries to prevent detection of automated browser control. The tool removes well-known DOM injection indicators that can reveal automation activities to websites.

## How It Works

The utility operates entirely in memory:

1. Loads the ChromeDriver binary into memory
2. Patches specific identifiers while preserving binary size
3. Executes the modified binary directly from memory using `fexecve()`
4. No temporary files are written to disk, avoiding filesystem write permission requirements

## Technical Background

### Anti-Detection Patches

These modifications hide automation indicators that websites commonly check for:

* [Stack Overflow: Selenium ChromeDriver cdc_ detection](https://stackoverflow.com/questions/33225947/)
* [What is the cdc_ variable in ChromeDriver?](https://dev.to/tonetheman/what-is-the-cdc-variable-in-chromedriver-553p)

### Memory-Only Execution Technique

The in-memory execution approach is based on:

* [In-Memory-Only ELF Execution (Without tmpfs)](https://magisterquis.github.io/2018/03/31/in-memory-only-elf-execution.html)
* [memfd_create(2) - Linux manual page](https://man7.org/linux/man-pages/man2/memfd_create.2.html)
* [fexecve(3) - Linux manual page](https://man7.org/linux/man-pages/man3/fexecve.3.html)

## Development

### Platform Compatibility

The core functionality (`memfd_create` and `fexecve`) is Linux-specific. However, for development on other platforms, the tool includes a fallback implementation using temporary files and the standard `execve` syscall.

### Building for Linux from Other Platforms

To compile for Linux when developing on MacOS or other systems:

```bash
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.86.0 cargo build --release

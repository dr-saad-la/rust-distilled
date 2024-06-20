# Rust Environment Setup

# Rust Environment Setup

Rust is a modern and powerful systems programming language. To get started with Rust, you'll need to install it on your system.

This `setup.md` file provides instructions for setting up Rust on Windows, macOS, and Linux, as well as installing Rust for use with Jupyter Notebook using the evcxr tool. You can customize and expand this document as needed.

Here are instructions for various platforms:

## Installing Rust on Windows

1. Visit the official Rust website for Windows: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).
2. Download the `rustup-init.exe` installer.
3. Run the installer and follow the on-screen instructions.
4. Open a new command prompt or terminal window and type `rustc --version` to verify that Rust is installed.

## Installing Rust on macOS

1. Open a terminal.
2. Install Homebrew if you don't already have it. Follow the instructions at [https://brew.sh](https://brew.sh).
3. Install Rust using Homebrew:
4. To verify the installation, run `rustc --version` in the terminal.

## Installing Rust on Linux

1. Open a terminal.
2. Visit the official Rust website for Linux: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).
3. Follow the instructions for your Linux distribution. They typically involve running a command to install Rust using `rustup`.
4. To verify the installation, run `rustc --version` in the terminal.

### Checking the Installation

After the installation has been completed successfully, you will have on your command line tool four new commands, you can check their versions as follows:

1. `rustup`: Rust installation tool manager
    ```
    rustup --version
    ```
2. `rustc`: Rust Compiler

    ```
    rustc --version
    ```

3. `rustdoc`: Rust documentation tool

    ```
    rustdoc --version
    ```

4. `cargo`: Rust compilation and package manager

    ```
    cargo --version
    ```

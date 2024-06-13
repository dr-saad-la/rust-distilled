<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
  Building Rust Projects with Cargo build
</div>

## Introduction

Cargo is the Rust package manager and build system that simplifies the process of managing Rust projects. It handles tasks such as building code, managing dependencies, running tests, and more. In this chapter, we will focus on the `cargo build` command, which is essential for compiling Rust code into executable binaries.

## Understanding Cargo Build

The `cargo build` command is used to compile your Rust project. It creates an executable binary from your source code, which you can run on your machine. This command ensures that your code and its dependencies are compiled correctly.

### Basic Usage

To build a Rust project, navigate to the project directory and run:

```bash
cargo build
```

- This command compiles the project in debug mode by default, producing an executable binary that includes debugging information.

## Debug vs Release Builds

- Cargo supports two build profiles: debug and release.

1. **Debug Build**:
    - Which is the default build mode.Includes debugging information.
    - Optimized for fast compilation and ease of debugging.
    - The executable binary is located in the target/debug directory.
  
	```bash
	cargo build
	```

2. **Release Build:**
    - Optimized for performance.
    - Takes longer to compile compared to the debug build.
    - The executable binary is located in the target/release directory.
 
To create a release build, use the `--release` flag:
	
	```bash
	cargo build --release
	```

## Understanding the Build Directory Structure

- After running `cargo build`, Cargo creates several directories and files in the target directory:
    - **target/debug**: Contains the debug build of your project.
    - **target/release**: Contains the release build of your project.
    - **target/.fingerprint**: Stores metadata used by Cargo to determine if files need to be rebuilt.
    - **target/deps**: Contains compiled dependencies of your project.
    - **target/build**: Contains build script output.

## Building Specific Targets

- In a Rust project, you might have multiple targets, such as libraries, binaries, and examples. You can specify which target to build using the --bin, --lib, or --example flags.

Building a Specific Binary:

	```bash
	cargo build --bin <binary-name>
	```

Building the Library:

	```bash
	cargo build --lib
	```

Building an Example:

	```bash
	cargo build --example <example-name>
	```

**Incremental Builds**: Cargo supports incremental builds, which means it only recompiles the parts of your code that have changed. This feature significantly speeds up the build process, especially for large projects. Incremental builds are enabled by default.

## Common Build Options

1. **Verbose Output**: Use the --verbose flag to get more detailed output during the build process.

	```bash
	cargo build --verbose
	```

2. **Clean Builds**: To remove the target directory and force a clean build, use the cargo clean command followed by cargo build.
	
	```bash
	cargo clean
	cargo build
	```

3. **Environment Variables**: Cargo allows you to set environment variables to control the build process. Some common environment variables include:

    - `CARGO_TARGET_DIR`: Changes the output directory of the build.

	```bash
	export CARGO_TARGET_DIR=custom_target_directory
	cargo build
	```

    - `RUSTFLAGS`: Passes additional flags to the Rust compiler.

	```bash
	export RUSTFLAGS="-C target-cpu=native"
	cargo build
	```

## Practical Example

- Let's create a simple Rust project to demonstrate the use of cargo build.

- Step 1: Create a New Project

```bash
cargo new simple_app
cd simple_app
```

- Step 2: Write Some Code
    - Edit the src/main.rs file:

```rust
fn main() {
    println!("Hello, world!");
}
```

- Step 3: Build the Project
    - Debug Build:

```bash
cargo build
```

```text
Compiling simple_app v0.1.0 (/path/to/simple_app)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

- Release Build:

```bash
cargo build --release
```

```text
Compiling simple_app v0.1.0 (/path/to/simple_app)
    Finished release [optimized] target(s) in 0.75s
```

- Step 4: Run the Executable 
	- Debug Build (Unix-like Systems)

	```bash
	./target/debug/simple_app
	```
	- Windows systems 
	```bash
	.\target\debug\simple_app.exe
	```
	- Release Build (Unix-like systems):

	```bash
	./target/release/simple_app
	```

	- Windows 
	```bash
	.\target\release\simple_app.exe
	```

### Summary

- The cargo build command is a powerful tool that simplifies the process of compiling Rust projects. It ensures that the code is compiled efficiently and correctly regardless of the project size. Hence, mastering this command options simplifies the development process a great deal. 

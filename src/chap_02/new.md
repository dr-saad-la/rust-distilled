<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">  
    The Cargo new Command
</div>

## The `new` Command

The `new` command in `Cargo` is essential for initializing new Rust projects. It sets up the necessary directory structure and files, allowing you to start development quickly and efficiently. As part of Cargo, Rust's package manager and build system, `cargo new` streamlines the process of creating both binary and library projects.

### Using the `new` Command in Cargo

The `cargo new` command is used to create a new Rust project. It can generate both binary and library projects. The basic usage of the command is as follows:

```bash
cargo new [OPTIONS] <path>
```

## Checking the `cargo new` Command Help Manual Page

When working with the command line, the `help` command is an invaluable resource. To check the manual for the `cargo new` command, use the following command in your terminal:

```bash
cargo help new
```

You will see an output similar to the one below. To navigate through the manual, you can use the following options:

  * Press the Return key (Enter) to move line by line.
  * Press the Space key to browse the manual page by page.
  * Press `q` to quit the manual page

```text
CARGO-NEW(1)                              General Commands Manual                             CARGO-NEW(1)

NAME
       cargo-new — Create a new Cargo package

SYNOPSIS
       cargo new [options] path

DESCRIPTION
       This command will create a new Cargo package in the given directory. This includes a simple
       template with a Cargo.toml manifest, sample source file, and a VCS ignore file. If the directory is
       not already in a VCS repository, then a new repository is created (see --vcs below).

       See cargo-init(1) for a similar command which will create a new manifest in an existing directory.

OPTIONS
   New Options
       --bin
           Create a package with a binary target (src/main.rs).  This is the default behavior.
       --lib
           Create a package with a library target (src/lib.rs).
       --edition edition
:
```

If you are already familiar with reading manuals or prefer to read them for your command-line applications, you can skip this entire chapter or simply skip to the end of the lesson to explore different scenarios and practical cases.

## Options for cargo new
 * `--bin`: Create a binary (executable) project. This is the default option.
 * `--lib`: Create a library project.
 * `--edition`: Specify the Rust edition (e.g., 2018, 2021).
 * `--name`: Set the package name (if different from the project directory name).
 * `--vcs`: Initialize a version control system repository (e.g., git). By default, Cargo initializes a git repository if the .git directory does not exist.

## Creating a New Binary Project

By default, `cargo new` creates a new binary project. This type of project includes the `main.rs` file, which serves as the entry point for the application.
    
```bash
cargo new <app_name>
```

This command creates a directory named simple_app with the following structure:

```text
app_name/
├── Cargo.toml
└── src/
    └── main.rs
```

 - `Cargo.toml`: The manifest file that contains metadata about the project, including dependencies.
 - `src/main.rs`: The main source file for the project, containing the main function.


## Creating a New Library Project
To create a new library project, use the `--lib` option:

```bash
cargo new simple_lib --lib
```

This command creates a directory named simple_lib with the following structure:

```text
simple_lib/
├── Cargo.toml
└── src/
    └── lib.rs
```
  - Cargo.toml: The manifest file for the project.
  - src/lib.rs: The main source file for the library.

### Naming Conventions for Rust Applications

When creating new Rust projects, it is important to follow the naming conventions to ensure consistency and readability across the Rust ecosystem. The preferred naming style is known as **snake_case**.

1. **Project Names**:
   - Use lowercase letters.
   - Words should be separated by underscores (`_`), following the snake_case convention.
   - Avoid using hyphens (`-`), numbers, or special characters.


   Examples:
   - `my_project`
   - `awesome_library`
   - `simple_app`

> It's worth noting that some users name their applications using a dash (`-`) to separate multiple words in Rust application names. While the compiler does not issue warnings in this case, it is recommended to use underscores (`_`) instead, as this follows the conventional snake_case style in Rust.

2. **Package Names**:
   - Follow the same conventions as project names.
   - The package name is specified in the `Cargo.toml` file.

3. **Crate Names**:
   - Crates are the fundamental compilation units in Rust, and crate names should follow the same conventions as project and package names.
   - Ensure that the crate name is unique when publishing to the Rust package registry, crates.io.

4. **Module Names**:
   - Module names should also be in lowercase and use underscores to separate words, adhering to the snake_case convention.
   - Module names typically correspond to the filenames of the module files.

By adhering to these naming conventions and using the snake_case style, you can make your Rust projects more accessible and maintainable for yourself and other developers.


## Practical Example: Creating a Simple Application

Let's walk through a practical example of creating a new binary project named simple_app.

1. Open your terminal.

2. Run the following command to create the new project:

```sh
cargo new simple_app
```

This command will create the following project structure:

```text
simple_app
├── .git
│   ├── HEAD
│   ├── config
│   ├── description
│   ├── hooks
│   ├── info
│   ├── objects
│   └── refs
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

I used the `tree -La 2 simple_app` command on Mac machine to display the folder structure. As you see, this command sets git repository for use automatically. 

3. Navigate to the newly created project directory:

```
cd simple_app
```

4. Open the project in your favorite text editor or integrated development environment (IDE). You should see the following structure:

5. Open the src/main.rs file. You will see the default content:

```rust
fn main() {
    println!("Hello, world!");
}
```

6. Run the application using Cargo:

```sh
cargo run
```

You should see the output:

```text
Compiling simple_app v0.1.0 (path/to/simple_app)
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
     Running `target/debug/simple_app`
Hello, world!
```

## Customizing the New Project

You can customize the creation of the new project using additional options. For example, to create a new binary project with a specified edition and without initializing a git repository, use the following command:

```bash
cargo new simple_app --edition 2021 --vcs none
```

## Summary

- The `cargo new`, is a powerful tool for quickly setting up new Rust projects. It simplifies the process of starting a new binary executable or a library, and ensures that you have the necessary files and directory structure to begin development immediately. 

- Leveraging the options available with `cargo new`, you can customize your project setup to meet specific requirements and preferences.

In this lesson, we covered the basics of using `cargo new` to create both binary and library projects, and provided a practical example of creating and running a new binary project named simple_app. This foundational knowledge will help you efficiently start new Rust projects as you continue your journey with the Rust programming language.


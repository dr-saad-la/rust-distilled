<div style="text-align:center;font-size:22pt; font-weight:bold;color:white;border:solid black 1.5pt;background-color:#1e7263;">
    Rust Environment Setup On Major Platforms
</div>

To get started with Rust, you'll need to install it on your system. Therefore, in this chapter, I will provide the necessary instructions for setting up Rust on Windows, macOS, and Linux, as well as installing Rust for use with Jupyter Notebook using the `evcxr` tool. These detailed step-by-step instructions are provided to ensure a smooth installation process, especially for those who are new to setting up development environments. 

After installing Rust, we will also verify that essential tools such as `rustup`, `rustdoc`, and `rustfmt` are properly installed and configured.

Here are the instructions for various platforms:


## Installing Rust on Windows

To begin using Rust on Windows system, follow these detailed steps to ensure a smooth and successful installation:

1. **Visit the Official Rust Website**: Navigate to the official Rust website by visiting [https://forge.rust-lang.org/infra/other-installation-methods.html](https://forge.rust-lang.org/infra/other-installation-methods.html). This page provides comprehensive resources and the latest installer for Rust.

2. **Download the Installer**: On the website, locate and download the `rustup-init.exe` installer. This installer is the recommended method for installing Rust, as it provides a straightforward setup process and keeps your Rust installation up-to-date.

3. **Run the Installer**: Once the download is complete, run the `rustup-init.exe` file. Follow the on-screen instructions to install Rust. The installer will guide you through the necessary steps, including setting up the Rust toolchain and configuring your environment.

4. **Verify the Installation**: After the installation is complete, open a new command prompt or terminal window. To verify that Rust is installed correctly, type the following command:
   ```sh
   rustc --version
   ```

This command checks the Rust compiler version and confirms that the installation was successful.

### Installing Rust on Windows Platforms Using Command Line tools

If you prefer doing the tedious work from the command line application, this section is for you, you will be instructed how to install Rust from using a CLI application with different utilities, `choco`, `scoop` or `winget`

#### Installing Rust using Chocolatey (choco)

Chocolatey is a popular package manager for Windows that simplifies software installation.

**Install Chocolatey**: If you don't already have Chocolatey installed, follow the instructions on [Chocolatey's official website](https://chocolatey.org/install).

**Open Command Prompt as Administrator**: Right-click on the Start menu and select "Command Prompt (Admin)" or "Windows PowerShell (Admin)".

**Install Rust**: Run the following command to install Rust using Chocolatey:

```sh
choco install rust
```

Verify the Installation: Once the installation is complete, verify by running:

```sh
rustc --version
```

#### Installing Rust using Scoop

`Scoop` is another package manager for Windows that is known for its simplicity and ease of use.

**Install Scoop**: If you don't have Scoop installed, open PowerShell and run:

```sh
iwr -useb get.scoop.sh | iex
```

for more information about `scoop` setup check [this link](https://github.com/ScoopInstaller/Install#readme)

Install Rust: After installing Scoop, run the following command in PowerShell:

```sh
scoop install main/rustup
```

Initialize Rust: Initialize Rust by running:

```sh
rustup-init.exe
```

Verify the Installation: Verify the installation by running:

```sh
rustc --version
```

#### Installing Rust using Windows Package Manager (winget) on Windows 11

The Windows Package Manager (winget) is available on Windows 11 and makes it easy to install software.

**Open Command Prompt**: Open Command Prompt or PowerShell.

**Install Rust**: Run the following command:

```sh
winget install -e --id Rustlang.Rustup
```

Verify the Installation: Verify the installation by running:

```sh
rustc --version
```

## Installing Rust on macOS

This section provides detailed instructions for macOS users to install Rust, whether you are using an Intel-based Mac or one with an Apple Silicon (M1, M2 or M3 series) chip. Rust is fully supported on both types of architecture, ensuring you can leverage its power and efficiency regardless of your hardware.

### Steps to Install Rust on macOS

1. **Open a Terminal**: The Terminal application can be found in the Applications > Utilities folder. Alternatively, you can use Spotlight by pressing `Cmd + Space` and typing "Terminal".

2. **Install Homebrew**: Homebrew is a popular package manager for macOS that simplifies the installation of software. If you don't already have Homebrew installed, open Terminal and follow these instructions:

   ```sh
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

Follow the on-screen instructions to complete the installation. For detailed guidance, visit [Homebrew's official website](https://brew.sh/).

Install Rust using Homebrew: Once Homebrew is installed, you can easily install Rust. Run the following command in your Terminal:

```sh
brew install rust
```

Verify the Installation: After the installation is complete, it is important to verify that Rust has been installed correctly. You can do this by running the following command in the Terminal:

```sh
rustc --version
```

This command should display the version of Rust that has been installed, confirming that the installation was successful.

## Installing Rust on Linux

This section provides detailed instructions for installing Rust on various Linux distributions, including Debian-based systems such as Ubuntu and Debian, as well as Fedora. 

### Steps to Install Rust on Debian-based Systems (Ubuntu, Debian)

1. **Open a Terminal**: You can open a terminal by searching for "Terminal" in your application menu or by pressing `Ctrl + Alt + T`.

2. **Visit the Official Rust Website**: For reference and additional details, visit the official Rust website for Linux: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).

3. **Install Required Dependencies**: Before installing Rust, make sure your system is up-to-date and install necessary dependencies:

   ```sh
   sudo apt update
   sudo apt install build-essential curl
   ```

Download and Run the rustup Installer: Use the following command to download and run the rustup installer, which will install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation.

Configure Your Current Shell: After installation, configure your current shell to use the `cargo`, `rustc`, and other Rust tools without needing to restart:

```sh
source $HOME/.cargo/env
```

Verify the Installation: To ensure Rust is installed correctly, run the following command in your terminal:

```sh
rustc --version
```

### Steps to Install Rust on Fedora
Open a Terminal: You can open a terminal by searching for "Terminal" in your application menu or by pressing Ctrl + Alt + T.

Visit the Official Rust Website: For reference and additional details, visit the [official Rust website for Linux](https://www.rust-lang.org/learn/get-started).

Install Required Dependencies: Ensure your system is up-to-date and install necessary dependencies:

```sh
sudo dnf update
sudo dnf groupinstall 'Development Tools'
sudo dnf install curl
```

Download and Run the rustup Installer: Use the following command to download and run the rustup installer, which will install Rust:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation.

Configure Your Current Shell: After installation, configure your current shell to use the cargo, rustc, and other Rust tools without needing to restart:

```sh
source $HOME/.cargo/env
```

Verify the Installation: To ensure Rust is installed correctly, run the following command in your terminal:

```sh
rustc --version
```

### Checking the Installation

After successfully completing the Rust installation, you will have four new commands available in your command line tool. These commands are essential for managing your Rust environment, compiling code, generating documentation, and handling packages. It is important to verify that each of these commands is properly installed and configured.

1. **`rustup`: Rust Installation Tool Manager**

    `rustup` is the toolchain installer for the Rust programming language, which allows you to manage different versions of Rust and their associated components. It ensures that you always have the right version of Rust and its tools.

    To check the version of `rustup`, run the following command in your terminal:

    ```sh
    rustup --version
    ```

    This command will display the version of `rustup` installed on your system, confirming that the installation was successful.

2. **`rustc`: Rust Compiler**

    `rustc` is the Rust compiler, which compiles your Rust source code into executable binaries. It is the core component of the Rust toolchain and is used to compile your Rust programs.

    To check the version of `rustc`, run the following command in your terminal:

    ```sh
    rustc --version
    ```

    This command will output the version of the Rust compiler installed on your system. Verifying this ensures that the compiler is correctly installed and ready for use.

3. **`rustdoc`: Rust Documentation Tool**

    `rustdoc` is the tool used to generate documentation for Rust projects. It extracts documentation comments from your source code and produces HTML documentation. This tool is invaluable for creating and maintaining project documentation.

    To check the version of `rustdoc`, run the following command in your terminal:

    ```sh
    rustdoc --version
    ```

    This command will display the version of `rustdoc` installed on your system, confirming its availability for generating documentation for your Rust projects.

4. **`cargo`: Rust Compilation and Package Manager**

    `cargo` is the Rust package manager and build system. It handles project dependencies, builds your code, runs tests, and manages project metadata. `cargo` simplifies many tasks associated with Rust development, making it an essential tool for any Rust programmer.

    To check the version of `cargo`, run the following command in your terminal:

    ```sh
    cargo --version
    ```

    This command will output the version of `cargo` installed on your system. Ensuring that `cargo` is properly installed verifies that you can manage your Rust projects and dependencies effectively.

Running these commands make sure that the Rust toolchain is correctly installed and configured on your system. This verification step is crucial to ensure that your development environment is set up correctly and that you can begin working on Rust projects without any issues. If any of these commands do not produce the expected output, you may need to revisit the installation steps or consult the official Rust documentation for troubleshooting guidance.

## Conclusion

Setting up Rust correctly on your system is the first and crucial step in your journey to mastering this powerful systems programming language. In this section, we have provided detailed instructions for installing Rust on various operating systems, including Windows, macOS, and Linux, using different package managers and tools tailored to each platform.

We also emphasized the importance of verifying your installation by checking the versions of essential Rust tools: `rustup`, `rustc`, `rustdoc`, and `cargo`. Ensuring these tools are correctly installed and configured will provide a solid foundation for your Rust development environment.

Following the steps outlined in this section enables you to have a fully operational Rust setup, ready for you to dive into writing and compiling Rust code, managing your projects, and generating documentation. As you move forward, this robust setup will support your learning and development efforts, allowing you to focus on writing efficient and safe Rust programs.

With your environment ready, you are now prepared to explore the capabilities of Rust further. In the next chapters, we will delve into Rust’s syntax, data structures, control flow, and more, building upon the solid foundation you have established here.



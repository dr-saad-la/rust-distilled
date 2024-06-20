# Setup Rust Kernel

## Installing Jupyter Lab

You can skip this section if you already have jupyter lab (or notebook) installed on your machine. If not, you can choose one method to get things done. In the following subsection, I will focus on **jupyter lab**, the newer version of classic **jupyter notebook**.

1. **Using anaconda distribution**:

2. **Using Command Line Tools**:
   There is a chance that you don't want anaconda to be installed on you machine, in fact it takes a lot of space from the hard disk, especially the full version. If this is your case, then what you need is python to be installed, and then install **jupyter lab**. I assume you already have python, if not please do so.

Then it suffices only to run the following command to install jupyter lab:

```
pip install -U jupyterlab
```

3. **Installing Jupyterlab Desktop Application**

-   **Unix-based Systems**

    -   Mac OS: Using `brew` utility as follows:

        ```
        brew install jupyterlab
        ```

    -   Linux (Ubuntu): You need to install `snapd` first

        ```
        sudo apt update
        sudo apt install snapd
        ```

        Then you can simply use the following command:

        ```
        sudo snap install jupyterlab-desktop --classic
        ```

    -   Fedora Linux:

        1. Install `snapd`

            ```
            sudo dnf install snapd
            ```

        2. Create symbolic link
            ```
            sudo ln -s /var/lib/snapd/snap /snap
            ```
        3. Install the application
            ```
            sudo snap install jupyterlab-desktop --classic
            ```

-   **Windows**

```
winget install jupyterlab
```

Please check the [jupyterlab official link](https://github.com/jupyterlab/jupyterlab-desktop) if you have any problem with installation of you have a different operating system.

## Installing Rust Kernel for Jupyter Notebook

If you want to use Rust in Jupyter Notebook, you can use the `evcxr` tool, which provides Rust support for Jupyter:

1. Install `evcxr` using `cargo`, the Rust package manager

```
cargo install evcxr_jupyter
```

2. Once `evcxr` is installed, you can configure Jupyter Notebook to use it

```
evcxr_jupyter --install
```

3. Start Jupyter Notebook

4. Create a new Jupyter notebook and choose the "Rust" kernel to start writing Rust code.

Now you're all set to explore the power of Rust on Jupyter Notebook.

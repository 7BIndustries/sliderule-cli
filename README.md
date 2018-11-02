# sliderule-cli

## Introduction
This is a command line implementation of the open hardware (OSHW) [sliderule methodology](https://github.com/Mach30/sliderule) being developed by [Mach 30](http://mach30.org/).

***NOTE*** This is in the very early stages of development, and is not ready for widespread use.

## Installation

git and npm must be installed separately for sliderule-cli to work. Binaries for sliderule-cli are avaiable for Linux, Windows and MacOS.

### Windows
- [Install git](https://git-scm.com/download/win)
- [Install npm](https://www.npmjs.com/package/npm#windows-computers)
- Download the [latest Windows build](http://7bindustries.com/downloads/sliderule-cli_dev_Windows_20181102.zip) of sliderule-cli.
- Extract the archive to a convenient location (i.e. `C:\Users\[you]\`).
- Run the command with the full path (i.e. `C:\Users\[you]\sliderule-cli\sliderule-cli.exe`).

Optionally, it is possible to set an alias within Windows Powershell for the CLI to make it more convenient to run:
```bash
PS C:\> Set-Alias sr C:\Users\[you]\sliderule-cli\sliderule-cli.exe
```

Unfortunately, at this time the passphrase has to be removed from the private key for the CLI to work. Follow the instructions [here](https://www.simplified.guide/ssh/set-remove-passphrase) to do that.

### Linux

- [Install git](https://git-scm.com/download/linux)
- [Install npm](https://www.npmjs.com/package/npm#fancy-install-unix)
- Download the [latest Linux build](http://7bindustries.com/downloads/sliderule-cli_dev_Linux_x64_20181017_1.tar.xz) of sliderule-cli.
- Extract the archive to a location in the `PATH` environment variable (i.e. `~/bin`).
- Run the command with the full path (i.e. `~/bin/sliderule-cli/sliderule-cli`).

Optionally, it is possible to create a symbolic link to make it more convenient to run the CLI:
```bash
$ ln -s ~/bin/sliderule-cli/sliderule-cli ~/bin/sr
```

### MacOS

- [Install git](https://git-scm.com/download/mac)
- [Install npm](https://www.npmjs.com/package/npm#apple-macintosh-computers)
- Download the [latest MacOS build](http://7bindustries.com/downloads/sliderule-cli_dev_MacOS_20181017_1.zip) of sliderule-cli.
- Extract the archive to a location in the `PATH` environment variable (i.e. `~/Applications`).
- Run the command with the full path (i.e. `~/Applications/sliderule-cli/sliderule-cli`).

Optionally, it is possible to create a symbolic link to make it more convenient to run the CLI (assuming `~/bin` is in your PATH):
```bash
$ ln -s ~/Applications/sliderule-cli/sliderule-cli ~/bin/sr
```

## Usage

### Creating and Managing a Sliderule Component

Everything in Sliderule is a component, even the top level "project" component that holds all other components. In this way, your top level project can be used as component of another project later on. The workflow for creating, initializing and populating a new project is as follows.

1. Run `sliderule-cli create [name]` in a directory that you have access to, and that is not already a component directory.
2. Change (`cd`) into the newly created component directory, with the name provided as the `name` argument from step 1.
3. Use `sliderule-cli add [url]` to begin pulling remote components into the project.
4. Use `sliderule-cli create [name]` to create new local components within your project.
5. Add and change project source files in each local component as needed to complete your design.
6. Use `sliderule-cli download`to download the latest changes for your project/component and for any remote componenets.
7. Use `sliderule-cli upload` and provide a message when prompted to upload all of the changes made within the project's file structure to its remote repository. If the current project is not set up for a remote repository, the CLI will prompt for a URL. The remote repository must already exist, and is not created by the CLI.

### Command Listing
- `sliderule-cli create [name] [-s SOURCE_LICENSE] [-d DOCUMENTATION_LICENSE]` - Creates a new component.
  - If the current directory is not a component, a `name` directory is created in the current directory, assuming the user has write access. The new directory is then initialized as a new top-level Sliderule project component, with files and directories being created as needed to match the Sliderule methodology.
  - If the current directory is already a component, creates a new local component `name` from scratch and places it within the `components` directory of the current project.
- `sliderule-cli download [all | dependencies | component_url]` - Downloads updates for the Sliderule project in the current directory.
  - `all` (default) - Downloads all changes to the component and its dependencies, assuming the current directory holds a Sliderule component.
  - `dependencies` - Downloads updates for only dependency components, assuming that the current directory is a Sliderule component.
  - `component_url` - Makes a copy of an existing remote component at the given URL. This creates a new directory for the downloaded component. Unless a user is an owner or maintainer of the remote component's repository, the downloaded component is read-only.
- `sliderule-cli upload` - Asks for a message to attach to any changes, and uploads all project/component changes. If the current component directory has not been initilized for a remote repository, the user is prompted to enter the repository's URL.
- `sliderule-cli add [url]` - Downloads a remote component and installs it in the current project. Unless a user is an owner or maintainer of the remote component's repository, remote components are read-only. If using a git host such as GitHub, the https URL must be used, instead of the SSH link. The URL provided can be from any supported repository type, such as git on GitHub: https://github.com/m30-jrs/blink_firmware.git
- `sliderule-cli remove [name]` - Removes the named component from a project. The name can refer to either a local or remote component.
- `sliderule-cli refactor [name]` - Changes a local component to a remote component. This command will ask for a URL for the component to be pushed to. The specified URL must exist prior to running this command. The remote repository for the component is not created automatically at this time. Use the SSH link to the repository if hosted on GitHub, GitLab, Git* instead of the https link, and make sure to have your ssh keys set up correctly for your operating system.

## Compiling It Yourself

If [Rust is installed](https://www.rust-lang.org/en-US/install.html) on a Linux, Windows or Mac computer, running the following should build the program. Note that `make.sh` is a wrapper script around cargo that will copy extra files that the CLI needs to run properly.
### Linux
```
./make.sh build [--release]
```
Once the build has completed successfully, the sliderule-cli binary (sliderule-cli.exe on Windows) should be located in `sliderule-cli/target/debug/`. Supply the full path to the sliderule-cli binary to use it. Alternatively, add the path to sliderule-cli to the `PATH` environment variable.

## Running Tests

At this time, tests will only run in Linux and MacOS.

If [Rust is installed](https://www.rust-lang.org/en-US/install.html), running the following command will execute the tests. Note that `make.sh` is a wrapper script around cargo because some files need to be copied before the tests are run.
```
./make.sh test
```
At this time the tests are only designed to run on Linux and MacOS.

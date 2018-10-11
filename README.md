# sliderule-cli

## Introduction
This is a command line implementation of the open source hardware [sliderule methodology](https://github.com/Mach30/sliderule) being developed by [Mach 30](http://mach30.org/).

***NOTE*** This is in the very early stages of development, and is not ready for widespread use.

## Installation

git and npm must be installed separately for sliderule-cli to work. Binaries for sliderule-cli itself will be avaiable shortly for Linux, Windows and MacOS.

### Windows
- [Install git](https://git-scm.com/download/win)
- [Install npm](https://www.npmjs.com/package/npm#windows-computers)

### Linux

- [Install git](https://git-scm.com/download/linux)
- [Install npm](https://www.npmjs.com/package/npm#fancy-install-unix)

### MacOS

- [Install git](https://git-scm.com/download/mac)
- [Install npm](https://www.npmjs.com/package/npm#apple-macintosh-computers)

## Usage

### Creating and Managing a Sliderule Component

Everything in Sliderule is a component, even the top level "project" component that holds all other components. In this way, your top level project can be used as component of another project later on. The workflow for creating, initializing and populating a new project is as follows.

1. Create a version controlled repository for the project on a personal server or site like [GitHub](https://github.com).
2. Create a directory with the same name as the new repository, and change (`cd`) into that directory.
3. Run `sliderule-cli init`, and supply the URL of the repository created in step 1 when prompted.
4. Use `sliderule-cli add [url]` to begin pulling remote components into the project. The URL provided can be from any supported repository type, such as git on GitHub: https://github.com/m30-jrs/blink_firmware.git
5. Use `sliderule-cli create [name]` to create new local components within your project.
5. Add and change project source files in each component as needed to complete your design.
6. Use `sliderule-cli update`to pull the latest changes for any remote componenets, and update any auto-generated files within the project and place them in the `dist` directory.
7. Use `sliderule-cli upload` to add a message and upload all of the changes made within the project's file structure to the remote repository.

### Command Listing
- `sliderule-cli init` - Initializes a directory as a new Sliderule project, creating files and directories as needed to match the Sliderule methodology.
- `sliderule-cli update` - Updates the Sliderule project, pulling all remote component changes, and then rendering all documentation according to the source.
- `sliderule-cli upload` - Asks for a message to attach to any changes, commits and pushes a project to git. Assumes the master branch.
- `sliderule-cli create [name]` - Creates a new local component from scratch and places it within the `components` directory of the current project.
- `sliderule-cli add [url]` - Downloads a remote component and installs it in the current project. Unless a user is an owner or maintainer of the remote component's repository, remote components are read-only.
- _Future:_ `sliderule-cli remove [name]` - Removes the named component from a project. The name can refer to either a local or remote component.
- _Future:_ `sliderule-cli refactor [name]` - Changes a local component to a remote component. This command will ask for a URL for the component to be pushed to. *NOTE:* The specified URL must exist prior to running this command. The remote repository for the component is not created automatically at this time.

## Compiling It Yourself

If [Rust is installed](https://www.rust-lang.org/en-US/install.html) on a Linux, Windows or Mac computer, running the following should build the program.
```
cargo build
```
Once the build has completed successfully, the sliderule-cli binary (sliderule-cli.exe on Windows) should be located in `sliderule-cli/target/debug/`. Supply the full path to the sliderule-cli binary to use it. Alternatively, add the path to sliderule-cli to the `PATH` environment variable.

# sliderule-cli

## Introduction
This is a reference implementation for the open [sliderule methodology](https://github.com/Mach30/sliderule) being developed by [Mach 30](http://mach30.org/).

***NOTE*** This is in the very early stages of development, and is not ready for widespread use.

## Installation

Binaries will be avaiable shortly for Linux, Windows and MacOS.

If [Rust is installed](https://www.rust-lang.org/en-US/install.html) on a Linux, Windows or Mac computer, running the following should build the program.
```
cargo build
```
Once the build has completed successfully, the sliderule-cli binary (sliderule-cli.exe on Windows) should be located in `sliderule-cli/target/debug/`. Supply the full path to the sliderule-cli binary to use it. Alternatively, add the path to sliderule-cli to the `PATH` environment variable.

## Usage

### Create a Sliderule Component

Everything in Sliderule is a component, even the top level "project" component that holds all other components. In this way, your top level project can be used as component of another project later on. The workflow for creating, initializing and populating a new project is as follows.

1. Create a version controlled repository for the project on a personal server or site like [GitHub](https://github.com).
2. Create a directory with the same name as the new repository, and change (`cd`) into that directory.
3. Run `sliderule-cli create`, and supply the URL of the repository created in step 1 when prompted.
4. Use `sliderule-cli add_component` to begin placing components with the project. In this step sliderule-cli will ask you to supply the URL of a remote component, or a name for the component to create locally.
5. Add and change project source files in each component as needed to complete your design.
6. Use `sliderule-cli update`to pull the latest changes for any remote componenets, and update any auto-generated files within the project and place them in the `dist` directory.
7. Use `sliderule-cli upload` to add a message and upload all of the changes made within the project's file structure to the remote repository.

### Command Listing
- `sliderule-cli create` - Initializes a directory as a new Sliderule project, creating files and directories as needed to match the methodology.
- `sliderule-cli add_component` - Asks for a component name or URL. Downloads a remote component if a URL is suppplied, or adds a local component labeled with the name provided.
- `sliderule-cli update` - Updates a Sliderule project recursively, pulling all remote component changes, and then renders all documentation according to the source.
- _Future:_ `sliderule-cli upload -m [message]` - Commits and pushes a project to git recursively, assuming the master branch. _Future:_ It will commit and push all components recursively, dealing with only the components/repos the user has access to.
- _Future:_ `sliderule-cli clone [URL]` - Clones a Sliderule project recursively, downloading all components.
- _Future:_ `sliderule-cli remove_component [name]` - Removes the named component from a project. The name can refer to either a local or remote component.
- _Future:_ `sliderule-cli component refactor [name] [URL]` - Sets up and initializes a local component as a remote component. *NOTE:* The specified URL must exist prior to running this command. The remote repository for the component is not created automatically at this time.

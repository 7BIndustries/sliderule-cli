# sliderule-cli

## Introduction
This is a reference implementation for the open [sliderule methodology](https://github.com/Mach30/sliderule) being developed by [Mach 30](http://mach30.org/).

Binaries will be avaiable shortly for Linux, Windows and MacOS.

This is in the very early stages of development, and is not ready for widespread use.

## Usage

### Create a Sliderule Component

Everything in Sliderule is a component, even the top level "project" component that holds all other components. In this way, your top level project can be used as component of another project later on. The workflow for creating, initializing and populating a new project is as follows.

1. Create a version controlled repository for the project.
2. Create a directory with the same name as the new repository, and change (cd) into that directory.
3. Run `sliderule-cli create`, and supply the URL of the repository created in step 1 when prompted.
4. Use `sliderule-cli component add` and/or `sliderule-cli componenent create` to begin placing components with the project.
5. Add and change source files as needed in each component.
6. Use `sliderule-cli update` to update any auto-generated files within the project and place them in the `dist` directory, and to pull the latest changes for any remote componenets.
7. Use `sliderule-cli upload` to add a message and upload all of the changes made within the project's file structure to the repository.

### Command Listing
- `sliderule-cli create` - Initializes a directory as a new Sliderule project, creating files and directories as needed to match the methodology.
- `sliderule-cli clone [URL]` - Clones a Sliderule project recursively,downloading all components.
- `sliderule-cli upload -m [message]` - Commits and pushes a project to git recursively, assuming the master branch. _Future:_ It will commit and push all components recursively, dealing with only the components/repos the user has access to.
- `sliderule-cli update` - Updates a Sliderule project recursively, pulling root changes and component changes recursively. _Future:_ It will build BoMs and documentation from source.
- `sliderule-cli component` - Has subcommands that allow the addition, removal and modification of components in the directory tree. Components are treated as git submodules of the main project.
  - `sliderule-cli component create [name|url]` - Creates a new component from a template and sets its URL. If a name is specified instead of a URL, the component will be created locally. *NOTE:* A specified URL must exist prior to running this command. The remote repository for the component is not created automatically at this time.
  - `sliderule-cli component add [url]` - Adds an existing component via its URL. *NOTE:* The specified URL must exist prior to running this command, and cannot be an empty repository. Use the `create` command if adding a brand new component.
  - _Future:_ `sliderule-cli component remove [name]` - Removes a component from a project (deletes its submodule).
  - _Future:_ `sliderule-cli component refactor [name] [URL]` - Sets up and initializes a local component as a remote component. *NOTE:* The specified URL must exist prior to running this command. The remote repository for the component is not created automatically at this time.

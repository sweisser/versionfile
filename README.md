[![build](https://github.com/sweisser/versionfile/actions/workflows/build.yml/badge.svg)](https://github.com/sweisser/versionfile/actions/workflows/build.yml)
[![release](https://github.com/sweisser/versionfile/actions/workflows/release.yml/badge.svg)](https://github.com/sweisser/versionfile/actions/workflows/release.yml)

# versionfile
A little commandline tool to keep versions for different components in a small YAML file.

Use it together with Makefiles and Jenkins Pipelines.

## Usage

### Help
````
$ versionfile -h
````

````
versionfile 2.0.0
Stefan Weisser <stefan.weisser@gmail.com>
A little tool to keep track of your component versions in a small YAML file. To be used in
Makefiles, Jenkinsfiles or Shell Scripts

USAGE:
    versionfile [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Sets a custom config file [default: versions.yaml]

SUBCOMMANDS:
    add          Add components
    env          
    get          Get a components current version
    get-cargo    Get version from a Cargo.toml (Rust projects)
    help         Prints this message or the help of the given subcommand(s)
    list         
    major        Increase a components major version
    minor        Increase a components minor version
    patch        Increase a components patch version

````

### Setup

Create a versions.yaml in your projects root. This file will keep track of each components version number.
For example, take the following project, which consists of a 'server' and a 'client' component.
````
---
versions:
  server: 0.0.1
  client: 0.2.0
````

### Usage

Now, in your Makefiles, Shellscripts or Jenkinsfiles, you can query each components version.

Query the version for a single component:

    $ versionfile get 'server'
    0.0.1

    $ versionfile get 'client'
    0.2.0

Versions of a Rust projects Cargo.toml can also be read (but not modified):

    $ versionfile get-cargo .
    2.0.0


### Makefile usage

You can use versionfile to set variables in a Makefile:

````
VERSION_CLIENT:=$(shell versionfile get client)

example:
    echo "Client version is $(VERSION_CLIENT)"
````

### Environment variables

Or to set environment variables:

````
export VERSION_CLIENT=`versionfile get client`
````

### Increasing versions for a component

    $ versionfile major 'client'
    1.0.0

    $ versionfile minor 'client'
    1.1.0

    $ versionfile patch 'server'
    0.0.2

### Versionfile location

Versionfile will be searched in current directory, or you can specify the location explicitly: 

    $ versionfile -c ../version.yaml get 'server'
    0.0.2

### List components

    $ versionfile list

### Add a component

    $ versionfile add 'proxy'
    $ versionfile get 'proxy'
    0.0.1

(Or add them directly in the YAML file.)

### Populate environment variables

    $ eval "$(versionfile env)"

## Build

You need to have Rust installed (e.g. via https://rustup.rs/)

Then it's just the usual:

    $ cargo build --release


[![build](https://github.com/sweisser/versionfile/actions/workflows/build.yml/badge.svg)](https://github.com/sweisser/versionfile/actions/workflows/build.yml)
[![release](https://github.com/sweisser/versionfile/actions/workflows/release.yml/badge.svg)](https://github.com/sweisser/versionfile/actions/workflows/release.yml)

# versionfile
A little commandline tool to keep versions for different components in a small YAML file and make them available in your environment.

Use it together with Makefiles and Jenkins Pipelines.


![alt text](render.gif)

## Usage

### Help
````
$ versionfile -h
````

````
versionfile 2.1.0
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
    env          Generate script to populate environment variables
    get          Get a components current version
    get-cargo    Get version from a Cargo.toml (Rust projects)
    help         Prints this message or the help of the given subcommand(s)
    init         Create a new versions.yaml
    list         List all components and versions
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

You can also use the 'init' subcommand to create an empty versions.yaml:

    $ versionfile init


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

### Jenkins Pipeline

Example how versionfile can be used inside a pipeline to version your components:

    stage('Build Client Application') {
      agent any

      environment {
        CLIENT_VERSION = "${sh(returnStdout: true, script: './versionfile get client')}".trim()
      }

      steps {
        sh 'make client'
        archiveArtifacts artifacts: 'client.tar.bz2'

        nexusArtifactUploader artifacts: [
            [artifactId: 'client', classifier: 'release', file: 'client.tar.bz2', type: 'tar.bz2'],
          ],
          credentialsId: 'jenkins_nexus_upload_user',
          groupId: 'org.coolapp',
          nexusUrl: 'localhost:8081',
          nexusVersion: 'nexus3',
          protocol: 'http',
          repository: 'maven-releases',
          version: "${env.CLIENT_VERSION}"
      }
    }


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

Take care with the naming of the components if you intend to use the 'env' subcommand.
Some characters, like '-' will cause trouble with the generated export commands.

### Populate environment variables

    $ eval "$(versionfile env)"

## Build

You need to have Rust installed (e.g. via https://rustup.rs/)

Then it's just the usual:

    $ cargo build --release


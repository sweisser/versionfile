# versionfile
A little commandline tool to keep versions for different components in a small YAML file.

Use it together with Makefiles and Jenkins Pipelines.

## Usage

### Setup

Create a versions.yaml in your projects root.
````
---
versions:
  server: 0.0.1
  client: 0.2.0
````

Query the versions for the various components:
````
versionfile get 'server'

0.0.1
````
````
versionfile get 'client'

0.2.0
````

### Increasing versions for a component

````
versionfile major 'client'

1.0.0
````
````
versionfile minor 'client'

1.1.0
````
````
versionfile patch 'server'

0.0.2
````

### Versionfile location

Versionfile will be searched in current directory, or you can specify the location explicitly: 
````
versionfile -c ../version.yaml get 'server'

0.0.2
````

### Add a component
````
versionfile add 'proxy'
versionfile get 'proxy'
0.0.1
````

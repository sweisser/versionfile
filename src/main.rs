use clap::{AppSettings, Clap};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use semver::Version;
use std::process::exit;

/// A little tool to keep track of your component versions in a small YAML file.
/// To be used in Makefiles, Jenkinsfiles or Shell Scripts.
#[derive(Clap)]
#[clap(version = "1.0.7", author = "Stefan Weisser <stefan.weisser@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file.
    #[clap(short, long, default_value = "versions.yaml")]
    config: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0.0", author = "Stefan Weisser <stefan.weisser@gmail.com>")]
    Get(Get),
    GetToml(GetToml),
    Add(Add),
    List,
    Env,
    Major(Major),
    Minor(Minor),
    Patch(Patch),
}

/// Get a components current version.
#[derive(Clap)]
struct Get {
    /// The component inside the versions file.
    component: String,
}

/// Get version from a Cargo.toml
#[derive(Clap)]
struct GetToml {
    /// The directory to read the Cargo.toml from.
    #[clap(short, long)]
    dir: Option<String>,
}

/// Add components.
#[derive(Clap)]
struct Add {
    /// The component inside the versions file.
    component: String,
}

/// List all components and versions.
#[derive(Clap)]
struct List {
}

/// Generate script to populate environment variables.
#[derive(Clap)]
struct Env {
}

/// Increase major version components.
#[derive(Clap)]
struct Major {
    /// The component inside the versions file.
    component: String,
}

/// Increase minor version components.
#[derive(Clap)]
struct Minor {
    /// The component inside the versions file.
    component: String,
}

/// Increase patch version components.
#[derive(Clap)]
struct Patch {
    /// The component inside the versions file.
    component: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut version_file: VersionFile = read_yaml(&opts.config);

    match opts.subcmd {
        SubCommand::List => {
            version_file.list();
        }
        SubCommand::Get(t) => {
            if let Some(version) = version_file.get(&t.component) {
                println!("{}", version);
            }
        }
        SubCommand::GetToml(t) => {
            if let Some(dir) = t.dir {
                if let Some(version) = read_version(&dir) {
                    println!("{}", version);
                } else {
                    println!("Error");
                }
            }
        }
        SubCommand::Add(t) => {
            version_file.add(&t.component);
            write_yaml(&opts.config, &version_file);
        }
        SubCommand::Major(t) => {
            version_file.inc(&t.component, increment_major);
            write_yaml(&opts.config, &version_file);
        }
        SubCommand::Minor(t) => {
            version_file.inc(&t.component, increment_minor);
            write_yaml(&opts.config, &version_file);
        }
        SubCommand::Patch(t) => {
            version_file.inc(&t.component, increment_patch);
            write_yaml(&opts.config, &version_file);
        }
        SubCommand::Env => {
            version_file.env();
        }
    }
}

fn read_yaml(configfile: &PathBuf) -> VersionFile {
    match std::fs::File::open(configfile) {
        Ok(file) => {
            let d: VersionFile = serde_yaml::from_reader(file).expect("Error parsing versionfile.");
            d
        }
        Err(e) => {
            eprintln!("Error opening versionfile {:?}: {}", configfile, e);
            exit(1);
        }
    }
}

fn write_yaml(configfile: &PathBuf, versions: &VersionFile) {
    use std::fs::OpenOptions;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(configfile)
        .expect("Couldn't open versionfile for writing.");

    let _res = serde_yaml::to_writer(file, versions);
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionFile {
    versions: HashMap<String, String>
}

fn increment_major(version: &mut Version) {
    version.increment_major()
}

fn increment_minor(version: &mut Version) {
    version.increment_minor()
}

fn increment_patch(version: &mut Version) {
    version.increment_patch()
}


impl VersionFile {
    pub fn get(&self, component: &str) -> Option<&String> {
        self.versions.get(component)
    }

    pub fn list(&self) {
        self.versions.iter().for_each(|(c, v)| {
            println!("{}: {}", c, v);
        })
    }

    pub fn env(&self) {
        self.versions.iter().for_each(|(c, v)| {
            let envvar_name = format!("VERSION_{}", c.to_uppercase());
            println!("export {}={}", envvar_name, v);
        })
    }

    pub fn add(&mut self, component: &str) {
        self.versions.insert(component.to_string(), "0.0.1".to_string());
    }

    pub fn inc<F>(&mut self, component: &str, versioning_operation: F)
        where F: Fn(&mut Version)
    {
        match self.versions.get(component) {
            Some(current_version) => {
                match Version::parse(current_version) {
                    Ok(mut bugfix_release) => {
                        versioning_operation(&mut bugfix_release);
                        self.versions.insert(component.to_string(), bugfix_release.to_string());
                        println!("{}", bugfix_release.to_string());
                    }
                    Err(e) => {
                        eprintln!("Couldn't parse version {} as semver: {}", current_version, e);
                    }
                }
            }
            None => {
                eprintln!("No version for component {}", component);
            }
        }
    }
}


fn read_version(dir: &str) -> Option<String> {
    let filename = PathBuf::from(dir).join("Cargo.toml");
    match cargo_toml::Manifest::from_path(&filename) {
        Ok(contents) => {
            match contents.package {
                Some(package) => {
                    Some(package.version)
                }
                None => None
            }
        }
        Err(e) => {
            eprintln!("Error reading {:?}: {}", &filename, e);
            None
        }
    }
}

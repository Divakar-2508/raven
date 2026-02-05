mod models;
mod raven;

use std::{env, io, path::PathBuf, process::Command};

use crate::models::{Package, PackageType};

fn main() {
    let mut args = env::args().skip(1);

    let cmd = match args.next() {
        Some(s) => s.trim().to_lowercase(),
        None => {
            eprintln!("Usage: raven <command>");
            return;
        }
    };

    match cmd.as_ref() {
        "add" => {
            let package_name = args.collect::<Vec<String>>().join(" ");
            add_dependency(&package_name);
        }
        "plug" => {
            let package_name = args.collect::<Vec<String>>().join(" ");
            add_plugin(&package_name);
        }
        "new" => {
            let name = match args.next() {
                Some(s) => s,
                None => {
                    eprintln!("Name not specified, raven new <project id/name>");
                    return;
                }
            };

            let grp = match args.next() {
                Some(s) => s,
                None => {
                    eprintln!("Group not specified, raven new <grp id/name>");
                    return;
                }
            };

            create_project(&name, &grp);
        }
        _ => {
            eprintln!("Invalid Command, Available: [add, new, plug]");
        }
    }
}

fn create_project(name: &str, grp: &str) {
    let goal = "archetype:generate";
    let artifact_id = format!("-DartifactId={name}");
    let grp_id = format!("-DgroupId={grp}");
    let template = "-DarchetypeArtifactId=maven-archetype-quickstart";
    let template_version = "-DarchetypeVersion=1.5";
    let interactive = "-DinteractiveMode=false";

    Command::new("mvn")
        .args([
            goal,
            &artifact_id,
            &grp_id,
            template,
            template_version,
            interactive,
        ])
        .spawn()
        .expect("Cannot Invoke mvn, Check if it's accessible")
        .wait()
        .expect("I dont really know");
}

fn add_dependency(package_name: &str) {
    let mut packages = raven::get_remote_packages(package_name, PackageType::Dependency).unwrap();

    let selected_package = get_choice(&mut packages);

    let pom_path = match get_pom_path() {
        Some(p) => p,
        None => {
            println!("pom.xml does not exists, cannot add Dependency");
            return;
        }
    };

    if let Err(err) = raven::write_deps_to_xml(selected_package, &pom_path) {
        eprintln!("Error: {err}");
    } else {
        println!("Added Dependency {} to the project", &selected_package.name);
    }
}

fn add_plugin(package_name: &str) {
    let mut packages = raven::get_remote_packages(package_name, PackageType::Plugin).unwrap();

    let selected_package = get_choice(&mut packages);

    let pom_path = match get_pom_path() {
        Some(p) => p,
        None => {
            println!("pom.xml does not exists, cannot add Dependency");
            return;
        }
    };

    if let Err(err) = raven::write_deps_to_xml(selected_package, &pom_path) {
        eprintln!("Error: {err}");
    } else {
        println!("Added plugin {} to the project", &selected_package.name);
    }
}

fn get_choice(packages: &mut [Package]) -> &Package {
    packages
        .iter()
        .enumerate()
        .for_each(|(idx, p)| println!("{}. {} ({})", idx + 1, p.name, p.group));

    let mut input = String::new();
    eprint!("Select Option (1): ");
    io::stdin().read_line(&mut input).unwrap();

    let choice: usize = input.trim().parse().map(|n: usize| n - 1).unwrap_or(1);
    let selected_package = packages.get_mut(choice).unwrap();

    raven::update_package_version(selected_package).unwrap();

    println!("{}", selected_package.to_xml_string());

    packages.get(choice).unwrap()
}

fn get_pom_path() -> Option<PathBuf> {
    let pom_path = env::current_dir()
        .unwrap_or(PathBuf::from("."))
        .join("pom.xml");

    if pom_path.exists() {
        Some(pom_path)
    } else {
        None
    }
}

// fn remove_package(package_name: &str) {}

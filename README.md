A quick-and-dirty tool for adding Maven dependencies faster, made for personal use.

# Raven

A small command-line helper to speed up common dependency/plugin tasks for Maven projects.

# Caution

It's a Lazy Work, don't really use unless you know what you are doing..

## Build 

Steps:
1. Clone the repository:
   ```
   git clone https://github.com/Divakar-2508/raven.git
   cd raven
   ```
2. Build with Cargo:
   ```
   cargo build --release
   ```
3. Run the tool using cargo or manual binary (target/release/raven) :
   ```
   <cargo run | raven> {command}
   ```

## Usage

Basic syntax:
```
raven <add|new|plug> {name}
```
new - create a new maven project, uses mvn "archetype:generate -DartifactId={name} -DgroupId={grp} -DarchetypeArtifactId=maven-archetype-quickstart -DarchetypeVersion=1.5 -DinteractiveMode=false"

add - adds a dependency, search by name in maven repo and asks for the exact package (from 10 relevant) to add.

plug - add a new plugin

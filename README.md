# Rust Project Template

## Overview
A template repository for Rust projects with Clippy and Bacon. A GitHub Action 
workflow is provided with the on `workflow_dispatch` event. This event has one type which is
`type`. `type` is the type of project will be generated. More information can be found below.

## Template Parameters: `type`
`type` determines how the project will be generated. The supported types are
Executable, and Library.

### Project Types: Executable
#### Rust Executable Project File Tree
```text
.
├── src/
│   └── main.rs
├── tests/
│   ├── 
│   └── 
├── .clippy.toml
├── .gitignore
├── build.rs
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```


### Project Types: Library
#### Rust Project File Tree
```text
.
├── src/
│   └── lib.rs
├── tests/
│   ├── 
│   └── 
├── .clippy.toml
├── .gitignore
├── build.rs
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

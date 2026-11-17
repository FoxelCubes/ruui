# RUUI
RUUI is a Rust framework I am currently experimenting with for creating cross-platform native UI apps.

I have only lightly touched on Svelte but the purpose of this project is to be a Svelte-inspired UI framework for Rust.
If you have any suggestions on how to improve the syntax this is still in the early days.

If I release any apps with this in use I will link this. If you decide to make anything also please raise a ticket and ill link it :)

## Project Structure
The repository is a Cargo workspace organized around a platform-independent UI
contract and separate rendering backends:

```text
ruui/
├── crates/
│   ├── ruui/                        Public API
│   ├── ruui-macros/                 tbd (need to explain this better)
│   ├── ruui-(platform)-(framework)/ Specific platform and framework bindings
│   ├── ruui-build/                  Project and Cargo tooling
│   └── ruui-cli/                    Cli tool to trigger ruui-build and other assistive tools
├── examples/                        Examples of how to use
└── Cargo.toml                       Workspace members and shared dependencies
```

## Testing Locally
Just create a new example :) or point to the repo locally. I'll add an example here later.

## Getting Started
While you may be able to cross compile for different platforms, 
for now the instructions will include getting them working on each base platform.

For all platforms you will need [Rust](https://rustup.rs/) to be installed.

Install the `ruui` cli tool via
```bash
cargo install --path crates/ruui-cli
```
To check if everything is set up you can run `ruui doctor` and it will the tools you need are detected.

### Desktop
#### macOS
Make sure that xcode is installed. Running `xcode-select --install` Should state its already installed.

### Mobile

### Wearable

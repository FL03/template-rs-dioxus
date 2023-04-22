# template-rs-dioxide

[![Clippy](https://github.com/FL03/template-rs-dioxide/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/template-rs-dioxide/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/template-rs-dioxide/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/template-rs-dioxide/actions/workflows/docker.yml)
[![Clippy](https://github.com/FL03/template-rs-dioxide/actions/workflows/pages.yml/badge.svg)](https://github.com/FL03/template-rs-dioxide/actions/workflows/pages.yml)
[![Rust](https://github.com/FL03/template-rs-dioxide/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/template-rs-dioxide/actions/workflows/rust.yml)

***

Welcome to template-rs-dioxide, a template for creating elegant user interfaces written in Rust leveraging Dioxide and TailwindCSS.

## Getting Started

### Building from the source

Make sure [Rust](https://www.rust-lang.org/tools/install) is installed properly on the host machine

```bash
rustup update
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

#### _Install the dev tools_

```bash
cargo install trunk wasm-bindgen-cli
```

#### _Clone the repository_

```bash
git clone https://github.com/FL03/template-rs-dioxide
```

### Docker

#### _Build the image locally_

```bash
docker buildx build --tag template-rs-dioxide:latest .
```

#### _Pull the pre-built image_

```bash
docker pull jo3mccain/template-rs-dioxide:latest
```

#### _Run the image_

```bash
docker run --name template-rs-dioxide -d -p 8080:80 jo3mccain/template-rs-dioxide:latest
```

### Usage

```rust

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)

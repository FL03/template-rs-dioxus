# curiosity

[![Clippy](https://github.com/FL03/curiosity/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/curiosity/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/curiosity/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/curiosity/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/curiosity/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/curiosity/actions/workflows/rust.yml)

***

Welcome to curiosity, a sandbox WebAssembly environment

## Getting Started

### Building from the source

#### _Clone the repository_

```bash
git clone https://github.com/FL03/curiosity
```

### Docker

#### _Build the image locally_

```bash
docker buildx build --tag curiosity:latest .
```

#### _Pull the pre-built image_

```bash
docker pull jo3mccain/curiosity:latest
```

#### _Run the image_

```bash
docker run \
    -p 8080:8080 \
    jo3mccain/curiosity:latest
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

# rust-yew-frontend

[An introduction to Yew.rs](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL) from [Brooks Builds](https://www.youtube.com/@BrooksBuilds).

## Prerequisites

As outlined in the Yew getting started guide, you will need to install the following dependencies:

```bash
rustup target add wasm32-unknown-unknown

cargo test --target wasm32-unknown-unknown

cargo install --locked trunk
cargo install -f wasm-bindgen-cli
```

## Getting started

```Running the application
trunk serve --open
```

## Linting

You can also set up pre-commit to run the linting steps automatically during the commit phase, the pre-commit pipeline can be set up by running the following command on the project root:

```bash
pre-commit install
```

To manually run it:

```bash
pre-commit run --all
```

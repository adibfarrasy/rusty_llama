# Rusty Llama - an AI chatbot written in Rust

## Installing Leptos

`cargo install cargo-leptos`


## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -D tailwindcss` - install `tailwindcss` for styling

## Running the project

Run the server with hot reload

`cargo leptos watch`

Run the CSS hot reload

`npx tailwindcss -i ./input.css -o ./style/output.css --watch`

## Setting up the LLM

This project uses [TheBloke's Vicuna LLM](https://huggingface.co/TheBloke/vicuna-7B-v1.3-GGML/blob/main/vicuna-7b-v1.3.ggmlv3.q8_0.bin). Other text-based LLMs should theoreticallly work with the correct prompting. Once the LLM is downloaded, you can run:
`touch .env && echo "MODEL_PATH=/path/to/the/model" > .env`

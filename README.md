# Fomi Standard Library (fomi-std-lib)

This is the core standard library plugin for the **Fomi** AI Assistant. It provides essential utilities designed to be lightweight, fast, and secure via WebAssembly context.

## Features

Currently implemented features:

* **Time Management**: Get formatted local server time.
* *(Planned)* **UUID Generation**: Generate v4 UUIDs.
* *(Planned)* **Random**: Generate random numbers within a range.

## Usage

This plugin communicates via JSON over WASM shared memory.

### Commands

#### `get_time`
Returns the current local time formatted as `YYYY-MM-DD HH:MM:SS`.

**Input:**
```json
{
  "command": "get_time"
}

Output:
Plaintext

Current time: 2026-02-10 14:30:00

Development
Prerequisites

    Rust (latest stable or nightly for 2024 edition)

    wasm32-wasi target:
    Bash

    rustup target add wasm32-wasi

Building

To compile the plugin into a .wasm file:
Bash

cargo build --target wasm32-wasi --release

The output file will be located at:
target/wasm32-wasi/release/fomi_std_lib.wasm
Installation

    Build the project.

    Copy the .wasm file and manifest.json into your Fomi plugins directory.
# radio-control-hal Repository

## Project Status
- **Work-in-Progress**: Early development stage with ongoing active development. Expect significant updates.
- **Contribution**: Highly welcome! Join us in shaping this project's future.

## Overview
The `radio-control-hal` crate is a `no_std` compatible hardware abstraction layer for radio communications, aimed at remote-controlled vehicles and devices. It provides modular traits for interfacing with various radio hardware, designed specifically for embedded systems.

## Features
- **Poll Channels**: Poll the number of available channels.
- **Data Polling**: Retrieve raw and normalized data from specific channels.
- **Error Handling**: Tailored error handling for robust radio communication management.
- **`no_std` Compatibility**: Ensures suitability for embedded environments.

## Getting Started
Add the crate to your project's dependencies:
```toml
[dependencies]
radio-control-hal = { git = "https://github.com/Ragarnoy/radio-control-hal", branch = "master" }

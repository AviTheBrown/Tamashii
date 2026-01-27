# Tamashii (魂)

<div align="center">
  <img src="assets/mascot.png" width="300" alt="Tamashii Mascot">
  <br>
  <br>
  <a href="https://github.com/AviTheBrown/Tamashii/actions"><img src="https://img.shields.io/github/actions/workflow/status/AviTheBrown/Tamashii/rust.yml?branch=main&label=build" alt="Build Status"></a>
  <a href="https://github.com/AviTheBrown/Tamashii/releases"><img src="https://img.shields.io/github/v/release/AviTheBrown/Tamashii" alt="Version"></a>
  <a href="https://github.com/AviTheBrown/Tamashii/blob/main/LICENSE"><img src="https://img.shields.io/github/license/AviTheBrown/Tamashii" alt="License"></a>
  <img src="https://img.shields.io/badge/rust-2024-orange" alt="Rust Version">
</div>

<br>

Tamashii is a lightweight, high-performance command-line utility for file integrity verification. Built with Rust and leveraging asynchronous I/O architectures, Tamashii provides a reliable method for ensuring that critical files remain unaltered and authentic.

## Overview

Tamashii (derived from the Japanese word for "soul") serves as a monitoring tool for filesystem integrity. It computes SHA-256 hashes of target files and persists them in a local metadata store. This enables users to perform subsequent verification checks to detect unauthorized modifications or data corruption.

## Features

- **Asynchronous Architecture**: Built on `compio` for efficient, non-blocking file operations.
- **Command-Line Interface**: Simple, structured command set for initialization, tracking, and verification.
- **Local Metadata Store**: Persistence is handled via a human-readable `.tamashii.json` schema in the project root.
- **Enhanced Diagnostics**: Provides clear, status-aware terminal output for verification results.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024 or later)

### Building from Source

```bash
git clone https://github.com/AviTheBrown/Tamashii
cd tamashii
cargo build --release
```

### Path Configuration (Optional)

To enable global access, move the binary to a directory in your system PATH:

```bash
mv target/release/tamashii /usr/local/bin/
```

## Usage

### 1. Initialization
Establish a new integrity database in the current working directory:
```bash
tamashii init
```

### 2. Tracking a File
Begin monitoring a specific file:
```bash
tamashii add <path/to/file>
```

### 3. Integrity Verification
Verify a single monitored file:
```bash
tamashii verify <path/to/file>
```

Execute a global verification check for all monitored files:
```bash
tamashii verify --all
```

### 4. Database Status
Retrieve a summary of current tracking status and database metadata:
```bash
tamashii status
```

## Use Cases

- **Configuration Auditing**: Monitor production environment variables and configuration files for unexpected changes.
- **Security Forensics**: Track critical system binaries and application source files to detect tampering.
- **Data Validation**: Confirm the integrity of large-scale datasets and media assets over long-term storage periods.

## Current Limitations

Tamashii is currently in active development. Please note the following implementation details:

- **Metadata Constraints**: The internal `FileRecord` currently constraints the stored file size metadata to a 255-byte capacity. This is a schema-level demonstration limit; hash verification remains functional for files of any size.
- **Local Persistence**: Database records are stored in cleartext JSON. Sensitive environments may require additional filesystem-level encryption.
- **OS Support**: While binary-compatible with most systems, certain asynchronous I/O optimizations are prioritized for Unix-like environments.

## License

This project is licensed under the MIT License.

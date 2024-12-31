# Kloud

Kloud is a Rust-based demo of a secure, end-to-end encrypted cloud storage service. Files are encrypted locally using **AES-GCM**.

## Setup and Usage

### Prerequisites
- Install Rust: [https://www.rust-lang.org](https://www.rust-lang.org)

### Installation
```bash
git clone https://github.com/James-Wirth/Kloud
cd kloud

cargo build
cargo run
```

### API Endpoints

1. **Upload File**
   - `POST /upload/{file_name}`
   - Body: Binary file content.

2. **Download File**
   - `GET /download/{file_name}`

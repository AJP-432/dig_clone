# rdig 🦀

`rdig` is a simple, modern command-line tool written in Rust that mimics the basic functionality of the classic `dig` command. It performs a DNS `A` record lookup for a given hostname using Cloudflare's public DNS server (1.1.1.1).

## About The Project

This project was created as a learning exercise to explore network programming in Rust. It serves as a lightweight alternative to `dig` for quick and easy DNS queries directly from your terminal.

**Features:**

- Queries for `A` records.
- Uses Cloudflare's fast and private DNS resolver by default.
- Provides output similar to `dig`, including query time and server information.
- Cross-platform: builds for Linux, macOS, and Windows.

---

## 🚀 Installation

The easiest way to use `rdig` is to download the latest pre-compiled binary for your operating system from the [**GitHub Releases page**](https://github.com/your-username/rdig/releases/latest).

1. Download the appropriate file for your system (e.g., `rdig-linux-amd64`, `rdig-macos-amd64`, or `rdig-windows-amd64.exe`).
2. (For Linux/macOS) Make the file executable:

```bash
chmod +x ./rdig-linux-amd64
```

3. Move the binary to a location in your system's `PATH` to make it globally accessible (e.g., `/usr/local/bin` on Linux/macOS).

---

## Usage

To perform a DNS lookup, simply provide a hostname as an argument:

```bash
rdig google.com
```

**Example Output:**

```bash
; <<>> rdig 1.0 <<>> google.com
; Querying DNS server: 1.1.1.1:53

;; QUESTION SECTION:
;google.com.              IN      A

;; ANSWER SECTION:
google.com.        300     IN      A       142.251.32.78

;; Query time: 9 msec
;; SERVER: 1.1.1.1#53
;; WHEN: Wed Jun 11 13:16:20 2025
```

---

## 🛠️ Building From Source

If you prefer to build the project yourself, you'll need to have the Rust toolchain installed.

1. **Clone the repository:**

```bash
git clone https://github.com/your-username/rdig.git
cd rdig
```

2. **Build in release mode:**

```bash
cargo build --release
```

3. **Run the compiled binary:**
   The executable will be located at `target/release/rdig`.

```bash
./target/release/rdig github.com
```

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

<h1 align="center">
  <img src="ui/extra/images/logo.png" width=200 height=200/><br>
  Uplink
</h1>
asdsad
<h4 align="center">Privacy First, Modular, P2P messaging client built atop Warp.</h4>sadasdsadaadsasdasdas

<br/>
asdadasdsadadad
Uplink is written in pure Rust with a UI in [Dioxus](https://github.com/DioxusLabs) (which is also written in Rust). It was developed as a new foundation for implementing Warp features in a universal application.

The goal should be to build a hyper-customizable application that can run anywhere and support extensions.
asadadada
![Uplink UI](https://i.imgur.com/X4AGeLz.png)

---

## Quickstart

To get running fast, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.


**Standard Run:**
```
cargo run --bin ui
```

**Rapid Release Testing:**
This version will run close to release but without recompiling crates every time.
```
cargo run --bin ui --profile=rapid
```

---


## Dependency List

**MacOS M1+**
| Dep  | Install Command                                                  |
|------|------------------------------------------------------------------|
| Build Tools| xcode-select --install |
| Homebrew | /bin/bash -c "\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" |
| Rust | curl --proto  '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh |
| cmake | brew install cmake |
| Protoc | brew install protobuf |
| ffmpeg | brew install ffmpeg |

a. For it works, we need to install ffmpeg -> brew install ffmpeg for MacOS
And for Windows, I followed the steps on this site here

**Windows 10+**
| Dep  | Install Command                                                  |
|------|------------------------------------------------------------------|
| Rust | [Installation Guide](https://www.rust-lang.org/tools/install) |
| Protoc | [Download](https://github.com/protocolbuffers/protobuf/releases/download/v22.asdsadadadd0/protoc-22.0-win64.zip) |
| ffmpeg | [Installation Guide](https://www.geeksforgeeks.org/how-to-install-ffmpeg-on-windows/) |
asdadadaddasda

**Ubuntu WSL (Maybe also Ubuntu + Debian)**
| Dep  | Install Command                                                  |asdadad
|------|------------------------------------------------------------------|
| Rust | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |<zx<x<zadasd
| Build Essentials | `sudo apt install build-essential` |
| pkg-config | `sudo apt-get install pkg-config` |
| alsa-sys | `sudo apt install librust-alsa-sys-dev` |
| libsoup-dev | `sudo apt install libsoup-3.0-dev` |
| protobuf| `sudo apt-get install protobuf-compiler` |
| Tauri Deps | `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev` |
| ffmpeg| `sudo apt-get install ffmpeg` |

## Contributing

All contributions are welcome! Please keep in mind we're still a relatively small team, and any work done to ensure contributions don't cause bugs or issues in the application is much appreciated.

Guidelines for contributing are located in the [`contributing_process.md`](docs/contributing_process.md).

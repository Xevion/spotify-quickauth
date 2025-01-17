> [!IMPORTANT]
> This application is not 'real'; I mostly created it to learn about building a CLI application in Rust with high-end CI/CD pipelines and easy to execute multi-platform scripts.
> Interestingly enough, I never really developed the program itself, just the scripts and the CI/CD pipeline.
> And by the time I was ready to write it, the `spotify-player` project had already solved the [primary issue](https://github.com/aome510/spotify-player/issues/201#issuecomment-2439565162) I was trying to address.
> Thus, I'm archiving this project, as it's no longer necessary.
>
> This project is still pretty cool though in it's own right, with multi-platform ephemeral binaries over `curl` commands, Windows support, Linux MUSL targets (smaller binaries), ARM64 MacOS support, binary deployment to GitHub pages, automatic GitHub release uploads, and a pretty decent README - this project has it all.

# spotify-quickauth

[![Build Status](https://github.com/Xevion/spotify-quickauth/workflows/build/badge.svg)](https://github.com/Xevion/spotify-quickauth/actions)
[![Testing Status](https://github.com/Xevion/spotify-quickauth/workflows/test/badge.svg)](https://github.com/Xevion/spotify-quickauth/actions)
[![Crates.io](https://img.shields.io/crates/v/spotify-quickauth.svg)](https://crates.io/crates/spotify-quickauth)
![Crates.io MSRV](https://img.shields.io/crates/msrv/spotify-quickauth)
![GitHub last commit](https://img.shields.io/github/last-commit/Xevion/spotify-quickauth)

<!-- TODO: Add testing status badge -->

A simple CLI-based application for creating a `credentials.json` file, used by `librespot` derived applications, such as [spotify-player][spotify-player], [spotifyd][spotifyd], and [raspotify][raspotify].

- One command, no compilation, all platforms (Windows, Linux, MacOS), ARM included
- Automatically places configuration files
- No dependencies, no installation, no fuss

## Quickstart

You can run this application without installing anything by using the following commands.

```bash
curl -sSL https://xevion.github.io/spotify-quickauth/run.sh | sh -s --
```

The default invocation is likely fine for most users, it will try to understand the available paths for `credentials.json` to be written to, and allow you to select them.

> [!NOTE]
> Automatic detection is dependent on the related software being installed and/or relevant configuration files being present.

For **Windows**, you can paste this command into PowerShell:

```powershell
iex (irm "https://xevion.github.io/spotify-quickauth/run.ps1")
```

## Usage

This application is dead simple to use. Just run the command, and it'll tell you to connect to a fake 'device' in your Spotify interface.

> [!NOTE]
> You must be connected to the same network running `spotify-quickauth`, as the `zeroconf` technology **does not work** across **networks** nor **proxies**.

Once you connect, the credentials file will be created, and you'll be prompted to select which location(s) to place it in. Even if none of the relevant `librespot` applications are detected or installed, you can specify manual locations, or the current working directory.

## Installation

Installation is not necessary to use this application, but if you're having trouble, want to compile it yourself, or are using it frequently, you might want to install it.

> [!NOTE]
> The scripts above can be given the `-K` or `--keep` flag to keep the downloaded binary. This will prevent repeated API calls to GitHub if you're using the script frequently within a short period.

### Pre-built Binaries

Binaries are always available for download from the [releases page][latestRelease], and they're the same ones used by the shell scripts above.

Currently, the following targets are available for download:

- x64 Linux (MUSL) `x86_64-unknown-linux-musl`
- ARM64 Linux (MUSL) `aarch64-unknown-linux-musl`
- ARMv7 Linux `armv7-unknown-linux-musleabihf`
- Intel MacOS `x86_64-apple-darwin`
- Apple Silicon MacOS `aarch64-apple-darwin`
- x64 Windows `x86_64-pc-windows-msvc`
- ARM64 Windows `aarch64-pc-windows-msvc`

Please [file an issue][new-issue] if you are on a platform that is not supported, or if you encounter issues with the binaries.

### Via `cargo-binstall`

> [!NOTE]
> If the package cannot be found for your target or fails to be downloaded for any reason, `cargo-binstall` will automatically fall back to building the package from source.

`cargo-binstall` is a tool that allows you to install binaries from crates.io without needing to compile them yourself.

```
cargo binstall spotify-quickauth
```

If you're curious where the binary comes from, `cargo-binstall` will likely pull the binary directly from the [latest release][latestRelease] by this repository, selecting the most appropriate target for your host.

### Manual Installation

If you'd like to use the shell script above to install the binary, you can use the `-S/--stop` flag to prevent the script from running the binary after downloading it. It implicitly applies the `--keep` flag too.

You'll need to move the binary yourself though:

```bash
curl -sSL https://xevion.github.io/spotify-quickauth/run.sh | sh -s -- -S
mv spotify-quickauth /usr/local/bin
```

Make sure your directory of choice is in your $PATH though!

### Building from Source

Don't want to run my funky shell script? No problem! You can build the application from source easily.

- You'll need `cargo`, the Rust build system and package manager. It's included with the Rust toolchain, which you can install from [rustup.rs][rustup]
- This is an early project, so the minimum supported version of Rust is not known. I'm developing on 1.81.0 though.

```bash
git clone https://github.com/Xevion/spotify-quickauth.git
cd spotify-quickauth
cargo install --path .
spotify-quickauth --help
```

If you have any troubles building the project

- Make sure you're using a target that's supported by the project (see above).
- Certain targets may require specfic linkers. For example,

[latestRelease]: https://github.com/Xevion/spotify-quickauth/releases/latest/
[spotify-player]: https://github.com/aome510/spotify-player
[spotifyd]: https://github.com/Spotifyd/spotifyd
[raspotify]: https://github.com/dtcooper/raspotify
[rustup]: https://rustup.rs
[git]: https://git-scm.com
[binstall]: https://github.com/cargo-bins/cargo-binstall
[quickinstall]: https://github.com/cargo-bins/cargo-quickinstall
[binstall-installation]: https://github.com/cargo-bins/cargo-binstall#installation
[new-issue]: https://github.com/Xevion/spotify-quickauth/issues/new

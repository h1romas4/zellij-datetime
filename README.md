# zellij-datetime

![](https://github.com/h1romas4/zellij-datetime/workflows/Build/badge.svg)

This plugin adds a date and time pane to [Zellij](https://zellij.dev/), a terminal workspace.

![zellij-01.png](https://raw.githubusercontent.com/h1romas4/zellij-datetime/main/docs/images/zellij-01.png)

This plugin was created for my Zellij studies. For this reason, the time zone settings and color schemes are not yet implemented. If you need to make changes, please build this repository to get the modified .wasm.

Since it is a WebAssembly/WASI build, it will probably work in all environments, including amd64, Arm.

## Setup Zellij plugin

```bash
# create configration directory
mkdir -p ~/.config/zellij/layouts/
mkdir -p ~/.config/zellij/plugins/
# export default layaut (Be careful not to overwrite your settings)
zellij setup --dump-layout default > ~/.config/zellij/layouts/default.kdl
# deploy plugin .wasm
cd ~/.config/zellij/plugins/
# download datetime plugin
wget -O zellij-datetime.wasm https://github.com/h1romas4/zellij-datetime/releases/download/v0.5.2/zellij-datetime.wasm
```

## Usage

Set the full path to `zellij-datetime.wasm` in the `~/.config/zellij/layouts/default.kdl` layouts file.

```bash
vi ~/.config/zellij/layouts/default.kdl
```

```kdl
layout {
    pane size=1 borderless=true {
        plugin location="file:/home/hiromasa/.config/zellij/plugins/zellij-datetime.wasm"
    }
    pane size=1 borderless=true {
        plugin location="zellij:tab-bar"
    }
    pane
    pane size=2 borderless=true {
        plugin location="zellij:status-bar"
    }
}
```

## Build

Development

```bash
# If you don't have Zellij installed already
cargo install zellij
# Add Rust target wasm32-wasi
rustup target add wasm32-wasi
# Git clone
git clone https://github.com/h1romas4/zellij-datetime
cd zellij-datetime
# Building the plugin
cargo build
# Running in Zellij
zellij -l plugin.kdl
```

Production

```bash
cargo build --release
cp -p target/wasm32-wasi/release/zellij-datetime.wasm ~/.config/zellij/plugins/
zellij
```

## License

MIT License

## Note

### Operation log in riscv64

At this time, RISC-V is not yet supported in Wasmer 2.3 used by Zellij. RISC-V has been supported since Wasmer 3.2.

```bash
$ uname -a
Linux lpi4a 5.10.113-gbb4691fe5572 # riscv64 GNU/Linux
$ arch
riscv64
$ rustc -V
rustc 1.65.0 #riscv64gc-unknown-linux-gnu
$ zellij -V
zellij 0.36.0
$ zellij
Error occurred in server:

  × Thread 'async-std/runtime' panicked.
  ├─▶ At /home/sipeed/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-compiler-cranelift-2.3.0/src/config.rs:73:45
  ╰─▶ construct Cranelift ISA for triple: Unsupported
  help: If you are seeing this message, it means that something went wrong.

        -> To get additional information, check the log at: /tmp/zellij-1001/zellij-log/zellij.log
        -> To see a backtrace next time, reproduce the error with: RUST_BACKTRACE=1 zellij [...]
        -> To help us fix this, please open an issue: https://github.com/zellij-org/zellij/issues
```

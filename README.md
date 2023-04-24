# zellij-datetime

![](https://github.com/h1romas4/zellij-datetime/workflows/Build/badge.svg)

This plugin adds a date and time pane to [Zellij](https://zellij.dev/), a terminal multiplexer.

![zellij-01.png](https://raw.githubusercontent.com/h1romas4/zellij-datetime/main/docs/images/zellij-01.png)

This plugin was created for my Zellij studies. For this reason, the time zone settings and color schemes are not yet implemented. If you need to make changes, please build this repository to get the modified .wasm.

Since it is a WebAssembly/WASI build, it will probably work in all environments, including amd64, Arm, and RISC-V.

## Setup Zellij plugin

```bash
# create configration directory
mkdir -p ~/.config/zellij/layouts/
mkdir -p ~/.config/zellij/plugins/
# export default layaut
zellij setup --dump-layout default > ~/.config/zellij/layouts/default.kdl
# deploy plugin .wasm
cd ~/.config/zellij/plugins/
# download datetime plugin
wget https://github.com/h1romas4/zellij-datetime/releases/download/v0.5.0/zellij-datetime.wasm
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
# Building the plugin
cd zellij-datetime
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

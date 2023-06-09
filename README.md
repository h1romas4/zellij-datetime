# zellij-datetime

![](https://github.com/h1romas4/zellij-datetime/workflows/Release/badge.svg) ![](https://github.com/h1romas4/zellij-datetime/workflows/Build/badge.svg)

This plugin adds a date and time pane to [Zellij](https://zellij.dev/), a terminal workspace.

![zellij-04.png](https://raw.githubusercontent.com/h1romas4/zellij-datetime/main/docs/images/zellij-04.png)

Zellij's plugin system leverages WebAssembly/WASI, and this plugin will also work with both amd64 and Arm in the same binary.

## WIP

- [x] Support for changing timezone by click or scroll on a pane.
- [x] Support for timezone definition files.
- [x] Binary size reduction.
- [ ] Improved parsing of configuration files.
- [x] Support for background color specification.
- [ ] When a Zellij session is detached and reattached, the plugin stops without getting drawing and timer events.
- [ ] Unnecessary borderlines appear when this plugin is placed at the bottom of the workspace with borderless=true.

## Require

* Zellij `0.37.2`

```
$ zellij -V
zellij 0.37.2
```

```
$ cat Cargo.toml | grep -A 2 dependencies
[dependencies]
zellij-tile = "0.37.2"
zellij-tile-utils = "0.37.2"
```

> https://zellij.dev/documentation/plugin-upgrading.html
>
> Upgrading a Plugin
>
> Since Zellij plugins using zellij-tile rely on shared data structures, currently one would need to compile a plugin against the corresponding zellij-tile package of the zellij version it is installed on.

## Setup

Preparation of Plug-in deployment destination:

```bash
# create configration directory
mkdir -p ~/.config/zellij/layouts/
mkdir -p ~/.config/zellij/plugins/
# If you have already created a layout file, you do not need to do the following.
# export default layaut (Be careful not to overwrite your settings)
zellij setup --dump-layout default > ~/.config/zellij/layouts/default.kdl
```
[Download zellij-datetime.wasm](https://github.com/h1romas4/zellij-datetime/releases/latest/download/zellij-datetime.wasm):

```bash
cd ~/.config/zellij/plugins/
# download datetime plugin
wget -O zellij-datetime.wasm \
    https://github.com/h1romas4/zellij-datetime/releases/latest/download/zellij-datetime.wasm
```

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

## Usage

- Timezone can be selected by left mouse click or scrolling.
- Insert a date/time string into current pane with a right mouse click.

https://github.com/h1romas4/zellij-datetime/assets/4337664/6ba30ce8-f1c5-4c32-9d00-18e2224b4c37

## Settings

Place the configuration file [`.zellij-datetime.kdl`](https://github.com/h1romas4/zellij-datetime/blob/main/.zellij-datetime.kdl) in the Zellij startup directory.

If you are running Zellij from `.bashrc`, it will be `~/.zellij-datetime.kdl`.

**Timezone**

```
timezone {
    define "UTC" 0
    define "PDT" -7
    define "JST" +9
}

defalut_timezone "JST"
```

**Color**

```
background_color "#202020"
```

## Build

Development

```bash
# If you don't have Zellij installed already
cargo install --locked zellij
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
# Install wasm-snip
cargo install wasm-snip
# Build
cargo build --release
# Remove debug symbles and replaces unreachable.
wasm-snip target/wasm32-wasi/release/zellij-datetime.wasm -o target/wasm32-wasi/release/zellij-datetime-snip.wasm
# Deploy plugin directory
cp -p target/wasm32-wasi/release/zellij-datetime-snip.wasm ~/.config/zellij/plugins/zellij-datetime.wasm
# Running in Zellij
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

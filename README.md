# zellij-datetime

![](https://github.com/h1romas4/zellij-datetime/workflows/Release/badge.svg) ![](https://github.com/h1romas4/zellij-datetime/workflows/Build/badge.svg)

This plugin adds a date and time pane to [Zellij](https://zellij.dev/), a terminal workspace.

![zellij-08.png](https://raw.githubusercontent.com/h1romas4/zellij-datetime/main/docs/images/zellij-08.png)

Zellij's plugin system leverages WebAssembly/WASI, and this plugin will also work with both amd64 and Arm in the same binary.

## Require

* Zellij `0.41.1` or later

```
$ zellij -V
zellij 0.41.1
```

## Setup

Preparation of Plug-in deployment destination:

```bash
# create configuration directory
mkdir -p ~/.config/zellij/layouts/
mkdir -p ~/.config/zellij/plugins/
# If you have already created a layout file, you do not need to do the following.
# export default layout (Be careful not to overwrite your settings)
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

Example - Basic layout:

```kdl
layout {
    pane size=1 borderless=true {
        plugin location="file:/home/hiromasa/.config/zellij/plugins/zellij-datetime.wasm"
    }
    pane size=1 borderless=true {
        plugin location="zellij:tab-bar"
    }
    pane
    pane size=1 borderless=true {
        plugin location="zellij:status-bar"
    }
}
```

Example - The same line as the other bar(s):

```kdl
layout {
    pane size=1 split_direction="vertical" {
        pane size="75%" borderless=true {
            plugin location="zellij:tab-bar"
        }
        pane size="25%" borderless=true {
            plugin location="file:/home/hiromasa/.config/zellij/plugins/zellij-datetime.wasm"
        }
    }
    pane
    pane size=1 borderless=true {
        plugin location="zellij:status-bar"
    }
}
```

## Usage

- Timezone can be selected by left mouse click or scrolling.
- Insert a date/time string into current pane with a right mouse click. (Require `enable_right_click true` plugin setting)

https://github.com/h1romas4/zellij-datetime/assets/4337664/6ba30ce8-f1c5-4c32-9d00-18e2224b4c37

## Plugin settings

```
layout {
    pane size=1 borderless=true {
        plugin location="file:/home/hiromasa/.config/zellij/plugins/zellij-datetime.wasm" {
            timezone1 "PDT/-9"
            timezone2 "UTC/0"
            timezone3 "CEST/+2"
            timezone4 "JST/+9"
            timezone5 "NPT/+5.75"
            default_timezone "JST"
            date_format "%Y/%m/%d %A"
            time_format "%I:%M %p"
            background_color "#0080a0"
            foreground_color "#ffffff"
            pane_color "#1a1b26"
            enable_right_click false
            arrow_separator1 ""
            arrow_separator2 ""
            arrow_separator3 ""
            padding_adjust 0
            text_align "right"
        }
    }
}
```

|  Key                 |  Format         | Default        | Note |
| -------------------- | --------------- | -------------- | ---- |
| `timezone1`          | `"name/offset"` | `"UTC/0"`      |      |
| `timezone[2-9]`      | `"name/offset"` | -              |      |
| `default_timezone`   | `"name"`        | `"UTC"`        |      |
| `date_format`        | [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) | `"%Y-%m-%d %a"` |      |
| `time_format`        | [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) | `"%H:%M"`       | The screen refreshes every minute. |
| `background_color`   | `"#color"`      | `"#0080a0"`    |      |
| `foreground_color`   | `"#color"`      | `"#ffffff"`    | It may be adjusted automatically depending on the `background_color`. |
| `pane_color`         | `"#color"`      | `"#1a1b26"`    |      |
| `enable_right_click` | bool            | `false`        | Right-clicking on the clock outputs the string format to stdin; Allow `PermissionType::WriteToStdin` permission when starting the plugin. |
| `arrow_separator1`   | `"string"`      | `""`          | Delimiter string on line. Only the first character. |
| `arrow_separator2`   | `"string"`      | `""`          | 📅 Only the first character. |
| `arrow_separator3`   | `"string"`      | `""`          | ⌚ Only the first character. |
| `padding_adjust`     | i32             | `0`            | It can be used to adjust left-justified padding. For example, adjusting the separator width if it is off by full-width. |
| `text_align`         | `"string"`      | `"right"`      | `right` or `left` or `center` |

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
# Remove debug symbols and replaces unreachable.
wasm-snip target/wasm32-wasip1/release/zellij-datetime.wasm -o target/wasm32-wasip1/release/zellij-datetime-snip.wasm
# Deploy plugin directory
cp -p target/wasm32-wasip1/release/zellij-datetime-snip.wasm ~/.config/zellij/plugins/zellij-datetime.wasm
# Running in Zellij
zellij
```

Development on Ubuntu 22.04 with container

- It can be tested on the main branch of Zellij.
- There is no confusion with the current Zellij session.
- You can test the operation of the plugin authority from the initial state.

```bash
git clone https://github.com/zellij-org/zellij.git
git clone https://github.com/h1romas4/zellij-datetime
# build Zellij (A newer version of protobuf-compiler is required)
cd zellij
cargo xtask build --release
# build zellij-datetime
cd ../zellij-datetime
cargo build
# Run with container (podman or docker)
podman run \
    --name zellij-datetime \
    --env SHELL=/usr/bin/bash \
    -v ../zellij/target/release/:/opt/zellij \
    -v .:/opt/zellij-datetime \
    -w /opt/zellij-datetime \
    -it --rm \
    mcr.microsoft.com/devcontainers/base:ubuntu-24.04 \
    /opt/zellij/zellij -l plugin.kdl
```

## License

MIT License

## WIP

- [x] Support for changing timezone by click or scroll on a pane.
- [x] Support for timezone definition files.
- [x] Binary size reduction.
- [x] Improved parsing of configuration files.
- [x] Support for background color specification.
- [ ] When a Zellij session is detached and reattached, the plugin stops without getting drawing and timer events. [#2575](https://github.com/zellij-org/zellij/issues/2575)
- [x] Unnecessary borderlines appear when this plugin is placed at the bottom of the workspace with borderless=true.
- [ ] Display is disturbed when the screen width is smaller than the display string.
- [ ] ~~Separate control of the line from control of datetime string generation.~~ Support for Zellij UI components. [Rendering a UI - Zellij User Guide](https://zellij.dev/documentation/plugin-ui-rendering.html)

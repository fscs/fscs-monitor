# Monitor to Display relevant information to the Display at HHU
![Build](https://github.com/fscs/fscs-monitor/actions/workflows/rust.yml/badge.svg)
## Overview
- [Installation](#installation)
- [TODOs](#todos)

<a id="installation"></a>
## Installation
**Important:** for this project to work it is required to disable CORS plugins are availabe:
- [Chrome](
https://chromewebstore.google.com/detail/cross-domain-cors/mjhpgnbimicffchbodmgfnemoghjakai)
- [Firefox](https://addons.mozilla.org/de/firefox/addon/cross-domain-cors/?utm_source=addons.mozilla.org&utm_medium=referral&utm_content=search)

Make shure you have installed the latest version of Rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then install the Dependencies and set the build-target to WebAssambly:
```
cargo install trunk
rustup target add wasm32-unknown-unknown
```
Build the app via trunk:
```
trunk build --release
```
Or serve it via trunk (note the Font is only linked correctly when you build the app not when serving):
```
trunk serve
```
<a id="todos"></a>
## TODOs:
- [ ] Refactor Code


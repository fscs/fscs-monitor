# Monitor to Display relevant information to the Display at fscs hhu

## Overview
- [Installation](#installation)
- [TODOs](#todos)

<a id="installation"></a>
## Installation
**Important:** for this project to work it is required to disable CORS
for Chrome a simple extension is availabe 
[Here](
    https://chromewebstore.google.com/detail/cross-domain-cors/mjhpgnbimicffchbodmgfnemoghjakai
)

Make shure you have installed the latest version of Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install the Dependencies and set the build-target to WebAssambly:
```
cargo install --path .
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
- [ ] Add ics inrtegration


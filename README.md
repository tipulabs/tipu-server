# <p align="center">Tipu-Server</p>
<p align="center">
  <img alt="icon" src="https://i.imgur.com/IhaVgsp.png" width="100" height="100"> 
</p>
<p align="center">✨ Sparkle that goes with: <a href="https://github.com/tipulabs/tipu" target="_blank">Tipu</a> </a></p>

[![Run on Repl.it](https://repl.it/badge/github/tipulabs/tipu-server)](https://repl.it/github/tipulabs/tipu-server)

<p align="center">
  <img alt="icon" src="https://i.imgur.com/cMdiytV.png" width="500" height="350"> 
</p>

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install)

Clone the source locally:
```
$ git clone https://github.com/tipulabs/tipu-server
$ cd tipu-server
```

```
cargo build
cargo run
```

Deploy on Heroku:
```
heroku create --buildpack emk/rust
echo "web: ./target/release/hello_rust" >> Procfile
heroku push origin master
```
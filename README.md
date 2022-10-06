# razermacos-rs

librazermacos bindings for Rust.

```toml
[dependencies]
razermacos = "0.1"
```

## librazermacos

This version currently requires that you have the librazermacos.so library on your load path. This typically involves building the library manually and copying the resulting file to either `/usr/local/lib` or `/usr/lib`.

```shell
git clone https://github.com/fcoury/razermacos-rs
cd razermacos-rs/librazermacos-sys
git submodule update --init
cd librazermacos
make
sudo cp librazermacos.so /usr/local/lib
```

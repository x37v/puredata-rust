# Pure Data Externals in Pure Rust

Does not require any C, builds entirely with Cargo.

See the [examples](examples/) for implementations based on the 
[HOWTO](https://github.com/pure-data/externals-howto) examples.

## Requirements

* [Rust and Cargo](https://www.rust-lang.org/tools/install)
* [Pure Data](https://puredata.info), to run, not to build.

## TODO

* Documentation
* Build scripts to create `.pd_linux`, `.pd_darwin` from the `.so`, `.dynlib` etc.
* Expose pointer methods
* Support more creation argument configurations
* Clean up macros
* [crates.io](https://crates.io/) release

## References

* [Pure Data](https://puredata.info)
* [HOWTO write an External for Pure Data](https://github.com/pure-data/externals-howto)

## Links

* [kmtr pd_ext_rust](https://github.com/kmtr/pd_ext_rust) Another Pure Data External solution for rust, requires C.
* [RustAudio](https://github.com/RustAudio)

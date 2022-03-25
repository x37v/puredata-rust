# Pure Data Rust Sys

The `pd-sys` crate provides Rust FFI bindings for [Pure Data](https://puredata.info/) aka *pd*, a graphical multi-media programming language.
The bindings are exclusively based on [m_pd.h](https://github.com/pure-data/pure-data/blob/master/src/m_pd.h) and most likely to be used
for building plugins aka *externals*.


## Compile time features

* **doubleprecision** - enable double precision *f64* floats. Experimental.
* **instance** - enable the pd *instance* interfaces. Experimental. Maybe only useful from within [libpd](https://github.com/libpd/libpd)?


## Examples

See [examples/helloworld/](examples/helloworld) for an example of how to use **pd-sys** to create an *external*.


## License

This project is licensed with the ["Modified BSD License"](LICENSE.txt)
Pure Data is licensed with the ["Standard Improved BSD License"](https://github.com/pure-data/pure-data/blob/master/LICENSE.txt)

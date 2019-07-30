# puredata-external-macros

At this point there is one macro `external!` that can wrap a `struct` and its `impl` blocks.
If the `struct` implements one of the external `trait`s from `puredata-external`, you should
have automatically generated trampolines and setup for your pure data external.

Currently, the puredata class is named after a lower case version the wrapped
`struct`'s name, so the example below will generate a class called *counter*.

```rust
external! {
    pub struct Counter {
      ...
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
...
        }
    }
}
```

  
## Atrributes

At this point there a few attributes you can add to an `impl` block to register methods
in the puredata space.

### #[bang]

```rust
external! {
    pub struct Counter {
    }
    ...

    impl Counter {
        #[bang]
        pub fn bang(&mut self) {
        }
    ...
    }
}
```

This will generate a `bang` method in the pure data space for your external.
If your external gets a `bang` on its left (aka hot) inlet, this method will
be called.

### #[sel]

A selector method, automatically generates the signature based on the method
signature.

There are 2 optional parameters:

1. `name="value"`
    This will ignore the name of the `rust` method and use the selector `value` to
    trigger this method.

2. `defaults=number`
    This will identify that `number` of the arguments, starting from the back
    should be filled in with default values if the argument isn't provided.

The example below shows 2 bound selector methods:

1. `|reset(` takes no arguments
1. `|set v(` takes 0 or 1 argument, if no arguments are provided, `v` will be `0.0`


```rust
external! {
    pub struct Counter {
    }
    ...

    impl Counter {
        #[sel]
        pub fn reset(&mut self) {
            self.count = 0isize;
        }

        #[sel(defaults=1, name="set")]
        pub fn foo(&mut self, v: puredata_sys::t_float) {
            self.count = v as isize;
        }
    ...
    }
}
```

## TODO

* Libraries of externals
* Allow explicit external naming
* Bind other types of methods


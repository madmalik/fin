# `fin` - finite, NaN-free floating point numbers for rust

Working with floats can be a bit of a pain in the backside, since floats can
carry errors conditions that are not handled by the type system.
In addition, rust does not implement the `Ord` trait for `f3` and `f64`. Which
is correct since a total ordering makes no sense in the face of `NaN`-values.

`fin` aims to improve on that situation as a zero-cost abstraction (in the sense
that the performance hit is not greater than manually checking conditions where
its nessesary)

## Usage

Add this to your `Cargo.toml` (since the fin project is very much in flux,
getting this package from github is the preferred way for now):
```
[dependencies]
fin = { git = "https://github.com/madmalik/fin.git" }
````

and this to your crate root:

```
extern crate fin;
```

## Principle

Fin uses session types to track invariants on floating point numbers.



## License
MIT

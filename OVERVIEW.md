# Overview

last updated: 2017-11-23

```
+-----------------------------+
| ^            ^   #~ ~ ~ ~ ~ |
|     o    ^    ^ #~ ~ ~ ~ ~ ~|
|# ^   \_     ^    #~ ~ ~ ~ ~ |
| ~#     _\____   #~ ~ ~ ~ ~ ~|
|~ ~#   /  |   \ #~ ~ ~ ~ ~ ~ |
| ~#    \_/     x #~ ~ ~###~ ~|
|~ ~###      ^      ####   ## |
+-----------------------------+
```

## General architecture
The central types are `Clean` and `Dirty` that are simple wrappers around their
type parameter. This type parameter is a `f64` or `f32`. The `Float` Trait from
`num-traits` is used to abstract over that.

The `Float` trait is adapted to our types to reflect the changed semantics.
This trait is called `CleanFloat` and is also defined in `lib.rs`. The
implementation is aided by a little bit of macro magic.

`trait_impls.rs` is a just bag of busywork to implement all the operations and
common traits to make `clean-float` usable.
Obviously, `error.rs` defines the error infrastructure, but a lot of the actual
work to create and report the errors happens in `lib.rs` and `trait_impls.rs`,
so if something is wrong with the error handling the fault will most likely be found
there.

## Error handling
This crate uses the `failure` crate.
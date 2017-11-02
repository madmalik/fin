# `fin` -- finite, NaN-free floiting point numbers

Working with floats can be a bit of a pain in the backside, since floats can
carry errors conditions (not a number and ininity from an overflow) and rust
does the correct thing and doesn't implement Ord.

`fin` aims to improve on that situation as a zero-cost abstraction (in the sense
that the performance hit is not greater than manually checking conditions where
its nessesary)


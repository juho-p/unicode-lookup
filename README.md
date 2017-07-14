# Unicode lookup

Find unicode characters by their names using Boyer-Moore substring search
algorithm.

## Why?

It's a toy program for me learn a bit of Rust and substrign searching
algorithms.

## Example usage:

No real documentation is provided! These examples must to be enough.

Finding some emoji:

    $ unicode-lookup sleepy
    😪 SLEEPY FACE
    $ unicode-lookup weary-cat-face
    🙀 WEARY CAT FACE

Giving multiple search words (all must match):

    $ unicode-lookup greek letter small pi
    ͳ GREEK SMALL LETTER ARCHAIC SAMPI
    π GREEK SMALL LETTER PI
    ϡ GREEK SMALL LETTER SAMPI
    ᴦ GREEK LETTER SMALL CAPITAL GAMMA
    ᴧ GREEK LETTER SMALL CAPITAL LAMDA
    ᴨ GREEK LETTER SMALL CAPITAL PI
    ᴩ GREEK LETTER SMALL CAPITAL RHO
    ᴪ GREEK LETTER SMALL CAPITAL PSI

Searching for single character:

    $ unicode-lookup A
    A LATIN CAPITAL LETTER A

## How do I get it?

You need Rust and Cargo. You should use rustup.rs to install.

* clone this repo
* `cd unicode-data`
* `cargo install` (or just `cargo build` if you for some reasone don't want to install)

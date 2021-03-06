# ASCIIfyer

[![Crates.io][crates-badge]][crates-url]
[![Docs]][docs-badge][docs-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/asciifyer.svg
[crates-url]: https://crates.io/crates/asciifyer
[docs-badge]: https://docs.rs/asciifyer/badge.svg
[docs-url]: https://docs.rs/asciifyer
[apache-badge]: https://img.shields.io/crates/l/asciifyer
[apache-url]: https://github.com/EbonJaeger/asciifyer/blob/main/README.md

This crate provides an easy way to turn a normal image into an ASCII picture. Supported image formats are the ones that the `image` crate supports; [see here](https://github.com/image-rs/image#supported-image-formats) for the list.

By default, images are resized to 250x250 pixels before turning them into ASCII. You can override this by passing a `Dimension` struct with a width and height. If the image is smaller than the dimensions, it will not be resized.

# Features

- Open an image file and convert the contents into an ASCII string

## Optional Features

- `fetch` - Enabling this feature allows the use of a function to download images from a URL.

# Examples

Here is a simple example for turning an image into ASCII. To try out the provided example, use `cargo run --example asciify /path/to/picture.png`

```rust
use asciifyer::{convert_to_ascii, Dimension};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Please enter a path to an image");

    let dimensions = Dimension::new(50, 50);
    let ascii = convert_to_ascii(&path, Some(dimensions));

    println!("{}", ascii);
}
```

Example output using the provided dolphin image:

```
    ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
  ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;/;;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;#@@@@0;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;/########@@;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;@############;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;0/#00#########/;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;///O0000#######0;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;////;0000000000##;;;;;;;;;
 ;;;;;;;;;;;;;;;/#;;;://///;000000000000/;;;;;;;;
 ;;;;;;;;;;;;;;;//0;;/////;;0000000000000;;;;;;;;
 ;;;;;;;;;;;;;;;/;0##////;;;;0000000000000@;;;;;;
 ;;;;;;;;;;;;;;;;;;;0////;;;;0000000/00####0;;;;;
 ;;;;;;;;;;;;;;:;;0/;///;;;;;00000///O///###;;;;;
 ;;;;;;;;;;;;;;;;#/////;;;;;;00000O/:;;;/0##;;;;;
 ;;;;;;;;;;;;;;;:////O/;;;;;;:000O/;;;;;;00#/;;;;
 ;;;;;;;;;;;;;;;///////;;;::;:00///;;;;;;O0.;;;;;
 ;;;;;;;;;;;;;;O/////;:;;@  :;0O///:;;;;/'O;;;;;;
 ;;;;;;;;;;;;;0/////;;;;;@@';;;O/O//::;,,;;;;;;;;
 ;;;;;;;;;;;;0/////;;;;;;O;;;;;OOO/;:,,:;;;;;;;;;
 ;;;;;;;;;;;/////;;;;;;.;;;;;;;0OO/,,';;;;;;;;;;;
 ;;;;;;;;;;;/////;;;;;;;;;;;;;;/',,,..//;;;;;;;;;
 ;;;;;;;;;;;////;;;;;;;/;;;;;;,,,,....:;;;;;;;;;;
 ;;;;;;;;;;///;;;;;;;;//;;;;:,,..,....;;;;;;;;;;;
 ;;;;;;;;;;;//;;;;;;;//,;;;,....:,..;;;;;;;;;;;;;
 ;;;;;;;;;;;//;;;;;;OO,.;......;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;/;;;;;0OO..:......;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;/;;;;000.:;......;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;:;;;00;::;......;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;:;;00.::;.....,;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;:;00 ;;;.....,;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;:00';;;.....;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;;0,;;;....;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;::;;;....;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;/:;:;....;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;#///;;::...;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;#0,,:;;:...;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;:.,,,;;...;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;0;..,,;:.;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;...,;:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;/:.:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
 ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
  ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
   ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
    ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
```

# License

Copyright © 2020 Evan Maddock maddock.evan@vivaldi.net

ASCIIfyer is available under the terms of the Apache 2.0 license.

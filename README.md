This is a Dyson Sphere Program blueprint tool.

Features:
* Dump blueprint to a JSON file.
* Undump a blueprint back.
* Print blueprint info.
* Replace items/recipes (partial).

## Building on linux

* Install rust and cargo.
* Run `cargo build`.
* Your binary is in `target/debug/dspbp`.

## Building on Windows

I dunno. I'm lazy.

## Binary releases

I dunno for now. I'm lazy.

## Usage

Start with `dspbp help`, go from there.

## Acknowledgements

* Thanks to johndoe31415 and his https://github.com/johndoe31415/dspbptk. He
  reverse-engineered DSP's custom hash and a bunch of blueprint stuff. 80% of
  this code is reimplementation of what he did.
* Guys at DSP wiki (the fandom one) for recipe IDs.

## TODOs

* Binary releases.
* Better support for replacing things:
  * Belt labels!
  * Inserter / splitter filters!
  * Traffic monitors, I guess?
* Changing blueprint info like description and icons.
* Prettier error handling, eventually.
* More blueprint edit actions, maybe?

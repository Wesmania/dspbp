This is a Dyson Sphere Program blueprint tool.

Features:
* Dump blueprint to a JSON file.
* Undump a blueprint back.
* Print blueprint info.
* Replace items/recipes (partial).

## Binary releases

Check the Releases tab. For Windows, grab `dspbp.exe`. For linux, `dspbp`.

## Usage

Windows users: dspbp is a commandline tool. To use it do the following:
* Start powershell.
* Navigate to where you downloaded dspbp. If it's your Downloads folder, run this:
  ```
  cd $HOME\Downloads
  ```
* Run `./dspbp.exe help`, go from there.

Linux users: same deal, but you probably already know how to use the terminal.
Start with `dspbp help`, go from there.

## Acknowledgements

* Thanks to johndoe31415 and his https://github.com/johndoe31415/dspbptk. He
  reverse-engineered DSP's custom hash and a bunch of blueprint stuff. 80% of
  this code is reimplementation of what he did.
* Guys at DSP wiki (the fandom one) for recipe IDs.
* Thanks to huww98 and his awesome blueprint visualizer at
  https://github.com/huww98/dsp_blueprint_editor for letting me borrow his code
  as I see fit.

## TODOs

* Better support for replacing things:
  * Belt labels!
  * Inserter / splitter filters!
  * Traffic monitors, I guess?
* Changing blueprint info like description and icons.
* Prettier error handling, eventually.
* More blueprint edit actions, maybe?
* A library interface. Maybe someone will want to use it as wasm?

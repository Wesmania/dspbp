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

## Examples

I want to dump bluperint info to JSON.

```
dspbp -i blueprint.txt dump
```

I have a blueprint "72 iron.txt" that smelts 72 iron and want to make a "36 titanium.txt" blueprint that smelts 36 titanium. To do that, I have to replace all instances of iron ore with titanium ore, and iron ingot recipes with titanium ingot recipes. I also want to set icon text to "36".
```
dspbp -i "72 iron.txt" -o "36 titanium.txt" edit -B IronOre:TitaniumOre,IronIngot:TitaniumIngot -t "36"
```

I want to upgrade the old blueprint with green belts and mark 1 smelters to blue belts and mark 2 smelters.

```
dspbp -i "old setup.txt" -o "new setup.txt" edit -b ConveyorBeltMkII:ConveyorBeltMkIII,ArcSmelter:PlaneSmelter
```

I want to list all item or recipe names, so I know what to pass to dspbp.
```
dspbp items
dspbp recipes
```

I want to print some info about a blueprint.
```
dspbp -i 'blueprint.txt' info
```

## Python bindings

See [here](https://pypi.org/project/dspbp/).

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
  * Inserter / splitter filters!
  * Traffic monitors, I guess?
* Changing blueprint info like icons.
* Prettier error handling, eventually.
* More blueprint edit actions, maybe?

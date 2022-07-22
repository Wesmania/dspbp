Dyson Sphere Program blueprint tool, python bindings.

## Features

* Load and save blueprint files.
* Replace blueprint items and recipes (partial).
* Upgrade buildings.
* Print blueprint info.

## Example usage

```
import dspbp
from dspbp import DSPItem

data = open("my_blueprint.txt", "rb").read()
bp = dspbp.load(data)

replacement = {
    DSPItem.IronOre: DSPItem.CopperOre,
    DSPItem.IronIngot: DSPItem.CopperIngot,
}
bp.replace_item(replacement)

open("edited_blueprint.txt", "wb").write(dspbp.save(bp))
```

## Brief reference

### `PyBlueprint`
Blueprint class.

### `DSPItem`
DSP item enum. Use `dir(DSPItem)` to get a list of all item types.

### `DSPRecipe`
DSP recipe enum. Use `dir(DSPRecipe)` to get a list of all recipes.

### `dspbp.load(data: bytes) -> PyBlueprint`
Load a blueprint from a `bytes` object.

### `dspbp.save(bp: PyBlueprint) -> bytes`
Turn a blueprint into a `bytes` object.

### `PyBlueprint.icon_text: str`
Icon text property. Use it to set blueprint text under the icons.

### `PyBlueprint.get_description(self) -> str`
Get blueprint description.

### `PyBlueprint.replace_item(self, r: dict[DSPItem, DSPItem])`
Replace items in the blueprint. Any item with a key in `r` gets replaced with a
corresponding value.

Supports belt labels, logistic station slots and item icons.

TODO: inserter filters, splitter filters, traffic monitors.

### `PyBlueprint.replace_item(self, r: dict[DSPRecipe, DSPRecipe])`
Replace recipes in the blueprint. Any recipe with a key in `r` gets replaced
with a corresponding value.

### `PyBlueprint.replace_both(self, r: dict[DSPItem, DSPItem])`
Replace both items and recipes. Items get converted into a most basic recipe.

### `PyBlueprint.replace_building(self, r: dict[DSPItem, DSPItem])`
Upgrade and downgrade buildings.

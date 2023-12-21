#!/usr/bin/env python3

# First, load items and recipes.

class Triple:
    def __init__(self, en_name, cn_name):
        number, en_name = en_name.strip().split(maxsplit=1)
        number_2, cn_name = cn_name.strip().split(maxsplit=1)

        number = int(number)
        number_2 = int(number_2)
        assert(number == number_2)

        self.number = number
        self.en_name = en_name
        self.cn_name = cn_name
        self.id = Triple._make_id(en_name)

    def _make_id(en_name):
        def niceify(s):
            for c in ["(", ")", ".", "-"]:
                s = s.replace(c, "")
            s = s[0].capitalize() + s[1:]
            return s

        return "".join(map(niceify, en_name.split()))


en_items = open("data/en/items.txt").readlines()
cn_items = open("data/cn/items.txt").readlines()
items = list(map(lambda x: Triple(x[0], x[1]), zip(en_items, cn_items)))

en_recipes = open("data/en/recipes.txt").readlines()
cn_recipes = open("data/cn/recipes.txt").readlines()
recipes = list(map(lambda x: Triple(x[0], x[1]), zip(en_recipes, cn_recipes)))

# Now, paste them in the proper location in src/data/enums.rs. We have comment markers to make it work.

enums_source = open("src/data/enums.rs")

def replace_enums(lines, items, enum_name):
    enum_start = f"//{enum_name} enum start"
    enum_end = f"//{enum_name} enum end"
    output = []
    state = "before"
    for line in lines:
        if state == "before":
            output += [line]
            if enum_start in line:
                state = "within"
                for i in items:
                    output += [f"    {i.id} = {i.number},\n"]
                continue
        elif state == "within":
            if enum_end in line:
                output += [line]
                state = "after"
        else:
            output += [line]
    return output

enums_source = replace_enums(enums_source, items, "DSPItem")
enums_source = replace_enums(enums_source, recipes, "DSPRecipe")

enums_file = open("src/data/enums.rs", "w")
for line in enums_source:
    enums_file.write(line)


# Now, create localizations.

def make_locale(file, items, enum_name, get_name):
    f = open(file, "w")
    f.write("&[\n")
    for i in items:
        f.write(f"    ({enum_name}::{i.id}, \"{get_name(i)}\"),\n")
    f.write("]\n")
    f.close()

make_locale("src/locale/data/en/items.rs", items, "DSPItem", lambda i: i.en_name)
make_locale("src/locale/data/cn/items.rs", items, "DSPItem", lambda i: i.cn_name)
make_locale("src/locale/data/en/recipes.rs", recipes, "DSPRecipe", lambda i: i.en_name)
make_locale("src/locale/data/cn/recipes.rs", recipes, "DSPRecipe", lambda i: i.cn_name)

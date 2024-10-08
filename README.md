# freefish
A fish tank for your terminal that can be easily customized by creating new animated tanks, fish, ducks, and crabs. These assets are supplied as properly formatted `.json` files. This format makes creating colored frames of ASCII art simple.

![example freefish](README.gif)

# Installation
1. Clone the repository
```
$ git clone https://github.com/daviddwk/rustyfish
$ cd rustyfish
```
2. Build freefish
```
$ cargo build --release
```
3. Move freefish

This is optional, but will allow you to call freefish more easily.
```
# cp target/release/freefish /usr/local/bin/freefish
```
4. Initialize freefish

freefish is initialized by running the binary with the init flag from from the cloned directory.
```
$ freefish -i
```
This creates the following folders, and populates them with the assets provided in the `config` folder.

- `~/.config/freefish/tanks`
- `~/.config/freefish/fish`
- `~/.config/freefish/ducks`
- `~/.config/freefish/crabs`

5. Test that freefish was initialized properly

```
$ ./target/debug/freefish -l
```

The following output should be seen if freefish was initialized properly, possibly showing even more assets.

```
FISH:
    angel
    clown
    guppy
TANKS:
    aquarium
    box
DUCKS:
    duck
CRABS:
    crab
```

freefish should now be setup and ready for use, but don't forget to try adding your own custom tanks and creatures to `~/.config/freefish` aswell!

# Usage
freefish is used to display a dynamic tank filled with various fish ducks and crabs. This is done by using the tank, fish, duck, and crab flags to specify available assets for display. These flags, and others, are further explained in this section.

```
$ ./freefish -t aquarium -f guppy clown guppy guppy angel -d duck -c crab
```

## Help

The help text can be displayed by using the `-h` or `--help` flags.

## Initializing

Initializing freefish using the `-i` or `--init` flags creates the following directories.
- `~/.config/freefish/tanks`
- `~/.config/freefish/fish`
- `~/.config/freefish/ducks`
- `~/.config/freefish/crabs`
  
freefish will copy available assets from `./config`, if available, to populate the aforementioned directories. freefish should be initialized from the cloned directory to utilize the provided assets, but feel free to add your own assets to `~/.config/freefish`'s subdirectories manually!

## Listing Assets

Available tanks, fish, and ducks are `.json` files placed into the appropriate subdirectories of `~/.config/freefsh/`. They can be listed using the `-l` or `--list` flag. The names of these assets can then be used to specify which tank to use and which fish and ducks to fill it with.
```
$ freefish -l
```

## Selecting a Tank
A tank is specified with the `-t` or `--tank` flag followed by the name of a single tank. If a tank is not provided, a blank tank that dynamically takes the size of your terminal will be used.
```
-t <tank>
```
Tank asset files are stored in `~/.config/freefish/tanks` and are `.json` files. These json files should contain the following key structure.

- `depth` (optional: defaults to 0)
- `foreground_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
- `background_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`

Creatures will swim in front of the background animation, but behind the foreground animation.

See an [example](#example-tank).

### depth
The `depth` key corresponds to a non-negative value that specifies the depth of the water surface. If this key is excluded the depth defalts to zero, placing the surface of the water at the top of the tank and allowing fish to swim anywhere. If a positive value is specified than the surface of the water will be placed `depth` lines down, leaving `depth` lines of "air" at the top of the tank where fish cannot swim. Ducks swim at the surface of the water, so it is important to give them space for where their heads peak above the water.

### foreground\_animation & background\_animation
The foraground\_animation and background\_animation [animations](#animations) should contain identically sized frames, but these animations NEED NOT have the same number of frames.

## Adding Fish

Fish are added to the tank using the `-f` or `--fish` flag followed by any number of fish names. The name of a fish may be used multiple times to add multiple of that fish to the tank. Those fish specified first will be rendered in front of those listed later. This flag is optional, but who wants a tank with no fish?
```
-f <fish_0> ... <fish_n>
```
Fish asset files are stored in  `~/.config/freefish/fish` and are `.json` files. These json files should contain the following key structure.
- `forward_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
- `flipped_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
     
See an [example](#example-fish).
 
### forward\_animation & flipped\_animation

The forward\_animation and flipped\_animation [animations](#animations) should contain an identical number of identically sized frames.

## Adding Ducks
Ducks are added to the tank using the `-d` or `--ducks` flag followed by any number of duck names. The name of a duck may be used multiple times to add multiple of that duck to the tank. Those ducks specified first will be rendered in front of those listed later. This flag is optional, but it shouldn't be.
```
-d <duck_0> ... <duck_n>
```
Duck asset files are stored in  `~/.config/freefish/fish` and are `.json` files. These json files should contain the following key structure.

- `buoyancy` (optional: defaults to 0)
- `forward_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
- `flipped_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`

See an [example](#example-duck).

### buoyancy

The `buoyancy` key corresponds to a value that specifies the number of lines of the duck that should appear above the surface of the water. If this key is excluded the buoyancy defaults to 0, so the top of the duck will be at the top layer of water. 

### forward\_animation & flipped animation

The forward\_animation and flipped\_animation [animations](#animations) should contain an identical number of identically sized frames.

## Adding Crabs
Crabs are added to the tank using the `-c` or `--crab` flag followed by any number of crab names. The name of a crab may be used multiple times to add multiple of that crab to the tank. Those crabs specified first will be rendered in front of those listed later. This flag is optional.
```
-c <crab_0> ... <crab_n>
```
Crab asset files are stored in  `~/.config/freefish/crabs` and are `.json` files. These json files should contain the following key structure.

- `forward_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
- `flipped_animation` (see [Animations](#animations))
    - `symbols`
    - `colors`
    - `highlights`
 
See an [example](#example-crab).

### forward\_animation & flipped animation

The forward\_animation and flipped\_animation [animations](#animations) should contain an identical number of identically sized frames.

## Adjusting Frame Speed

The delay between frames can be modified using the `-s` or `--speed` flag followed by the desired delay between frames in ms. The default delay is 200ms.
```
-s <delay_ms>
```

## Quitting

freefish can be stopped by pressing the `q` or `Esc` keys.


# Animations
Assets contain animations in their json files, which have the following key structure where the name of the animation varies.

- <animation_name>
    - `symbols`
    - `colors`
    - `highlights`

The `symbols` `colors`, and `highlights` keys each correspond to a list of frames, where each frame is a list of strings. Each key corresponds to a list of frames of the same length, each frame must have the same number of strings, and each string must be the same length.

See [examples](#examples).

### symbols

The symbols frames contains characters that make up the ASCII art of the animation. Any space in this portion will be transparent, and the background will be rendered in its place. Keep in mind that the `\` and `"` characters must be escaped using the `\` character in json strings. 

### colors & highlights

The `colors` and `highlights` frames contain specific characters that bring color to the corresponding characters of the `symbols` frames. `colors` specifies the colors of the corresponding `symbols` character, while `highlights` specifies the color with which the corresponding `symbols` character will be highlighted. The following characters are used in these frames to apply the specified color to the corresponding character or its highlight.

- `a` : DarkGrey
- `r` : Red
- `g` : Green
- `y` : Yellow
- `b` : Blue
- `m` : Magenta
- `c` : Cyan
- `w` : White

- `A` : Black
- `R` : DarkRed
- `G` : DarkGreen
- `Y` : DarkYellow
- `B` : DarkBlue
- `M` : DarkMagenta
- `C` : DarkCyan
- `W` : Grey

# Examples

## example fish
```
{
    "forward_animation": 
    {
        "symbols": 
        [
            ["  n ",
             "><v>"],

            ["  n ",
             "><^>"]
        ],
        "colors": 
        [
            ["  n ",
             "mmrm"],

            ["  n ",
             "mmrm"]
        ],
        "highlights": 
        [
            ["    ",
             "    "],

            ["    ",
             "    "]
        ]
    },
    "flipped_animation": 
    {
        "symbols": 
        [
            [" n  ",
             "<v><"],

            [" n  ",
             "<^><"]
        ],
        "colors": 
        [
            [" m  ",
             "mrmm"],

            [" m  ",
             "mrmm"]
        ],
        "highlights": 
        [
            ["    ",
             "    "],

            ["    ",
             "    "]
        ]
    }
}
```
## example duck
```
{
    "buoyancy": 1,      <--- one row of the duck above water level
    "forward_animation": {
        "symbols": [
            ["  ()-",
             "<.v) ",   <--- water level here
             " ^^  "],

            ["  ()<",
             "<.v) ",
             " ^^  "]
        ],
        "colors": [
            ["  wwy",
             "wwww ",
             " yy  "],

            ["  wwy",
             "wwww ",
             " yy  "]
        ],
        "highlights": [
            ["     ",
             "     ",
             "     "],

            ["     ",
             "     ",
             "     "]
        ]
    },
    "flipped_animation": {
        "symbols": [
            [">()  ",
             " (v.>",   <--- water level here
             "  ^^ "],

            ["-()  ",
             " (v.>",
             "  ^^ "]
        ],
        "colors": [
            ["yww  ",
             " wwww",
             "  yy "],

            ["yww  ",
             " wwww",
             "  yy "]
        ],
        "highlights": [
            ["     ",
             "     ",
             "     "],

            ["     ",
             "     ",
             "     "]
        ]
    }
}
```
## example crab
```
{
    "forward_animation": {
        "symbols": [
            ["  \\/  ",
             "=<``><",
             " \"  \" "],
            ["  \\/  ",
             "=<``><",
             " '''' "]
        ],
        "colors": [
            ["  rr  ",
             "rrrrrr",
             " r  r "],
            ["  rr  ",
             "rrrrrr",
             " rrrr "]
        ],
        "highlights": [
            ["      ",
             "      ",
             "      "],
            ["      ",
             "      ",
             "      "]
        ]
    },
    "flipped_animation": {
        "symbols": [
            ["  \\/  ",
             "><``>=",
             " \"  \" "],
            ["  \\/  ",
             "><``>=",
             " '''' "]
        ],
        "colors": [
            ["  rr  ",
             "rrrrrr",
             " r  r "],
            ["  rr  ",
             "rrrrrr",
             " rrrr "]
        ],
        "highlights": [
            ["      ",
             "      ",
             "      "],
            ["      ",
             "      ",
             "      "]
        ]
    }
}
```
## example tank
```
{
    "depth": 2,   <--- top 2 rows have no water
    "foreground_animation": 
    {
        "symbols": 
        [
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          "
            ]
        ],
        "colors": 
        [
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          "
            ]
        ],
        "highlights": 
        [
            [
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          "
            ]
        ]
    },
    "background_animation": 
    {
        "symbols": 
        [
            [
                "@--------@",
                "|        |",
                "|^^^^^^^^|",   <--- top row where fish can swim
                "|        |",
                "|        |",
                "|        |",
                "@--------@"
            ]
        ],
        "colors": 
        [
            [
                "YYYYYYYYYY",
                "Y        Y",
                "YbbbbbbbbY",
                "Y        Y",
                "Y        Y",
                "Y        Y",
                "YYYYYYYYYY"
            ]
        ],
        "highlights": 
        [
            [
                "yyyyyyyyyy",
                "y        y",
                "y        y",
                "y        y",
                "y        y",
                "y        y",
                "yyyyyyyyyy"
            ]
        ]
    }
}
```

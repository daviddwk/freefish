# rustyfish
A fish tank for your terminal that can be easily customized by creating new animated tanks, fish, and ducks. These assets are supplied as properly formatted `.json` files. This format makes creating colored frames of ASCII art simple.

```
:                                                                             :
:                                                                             :
:                 ()<                                                         :
:~    ~~~  ~ ~~0<.v)  ~~~~~~    ~~~    ~~   ~~~~~   ~~  ~~~~     ~~~~  ~  ~~~~:
:  ~~  o  `   00 ^^   ~~~~       ~~       `~`      ~~      ~~~~        ~~~`   :
:  o  o00~<vo<  0  `                 ~~~    ``                     ~~         :
:          o0 o 0,_ _" `     /          //'                                   :
:     o0    0   <_<)\,><}   `\    /    /.\\ /     ` `                         :
:    o  0   o  0  "  '    _   \  \  \ <`v~\<|                         `       :
:    o  00o  0  0       ><v> _/   \ / _\\_/ \                             `   :
:      o   0  o0  0    ><v> \|    /|_/  \      `     ,_ _"  |$11$             :
:       00 oo00 0           |/   / /  _             <_<)\,><}  _              :
:            00  0          /\ / // _/       ><v>     "#|'|#|_|#|  ``         :
:           vV 0           / / || |/ /                |####|####|             :
:         /vVv\vV\         \ |<v&@_&|      _   _   _  |###|||###|  _   _   _  :
:       // vvV  \v\      &@@&\ _@//\&@&   ><^>|<^><#|_|#########|_|#|_|#|_|#| :
:     /   \  vV  |V\\   &@&/\&@&&|__\@@&  |#################################| :
:    |     \ V Vv| v \  @&|\ |&@&&@&&@&&@ |#############/!!!!!\#############| :
:  | |     | v\ v \   \@&@| \|&&&@@@&&@&&@|###|||##<^><#|     |#######|||###| :
: /  /   /  \   V  \  &&@@| ||@&@&@&@&&&@&@###|||#######|     |#######|||###| :
```

# Installation
1. Clone the repository
```
$ git clone https://github.com/daviddwk/freefish.git
$ cd freefish
```
2. Build freefish
```
$ cargo build
```
3. Initialize freefish

freefish is initialized by running the binary with the init flag from from the cloned directory.
```
$ ./target/debug/freefish -i
```
This creates the following folders, and populates them with the assets provided in the `config` folder.

- `~/.config/freefish/tanks`
- `~/.config/freefish/fish`
- `~/.config/freefish/ducks`

4. Test that freefish was initialized properly

```
$ ./target/debug/freefish -l
```

The following output should be seen if freefish was initialized properly, possibly showing even more assets.

```
fish
 guppy.json
 angel.json
 clown.json
tanks
 box.json
 aquarium.json
ducks
 duck.json
```

freefish should now be setup and ready for use, but don't forget to try adding your own custom tanks and creatures to `~/.config/freefish` aswell!

# Usage
freefish is used to display a dynamic tank filled with various fish and ducks. This is done by using the tank, fish, and duck flags to specify available assets for display. These flags, and others, are further explained in this section.

```
$ ./freefish -t aquarium -f guppy clown guppy guppy angel -d duck
```

## Initializing

Initializing freefish using `-i` creates the following directories.
- `~/.config/freefish/tanks`
- `~/.config/freefish/fish`
- `~/.config/freefish/ducks`
  
freefish will copy available assets from `./config`, if available, to populate the aforementioned directories. freefish should be initialized from the cloned directory to utilize the provided assets, but feel free to add your own assets to `~/.config/freefish`'s subdirectories manually!

## Listing Assets

Available tanks, fish, and ducks are `.json` files placed into the appropriate subdirectories of `~/.config/freefsh/`. They can be listed using the `-l` flag. The names of these assets can then be used to specify which tank to use and which fish and ducks to fill it with.
```
$ freefish -l
```

## Selecting a Tank
A tank is specified with the `-t` flag followed by the name of a single tank.
```
-t <tank>
```
Tank asset files are stored in `~/.config/freefish/tanks` and are `.json` files. These json files should contain the following key structure.

- `depth` (optional: defaults to 0)
- `foreground` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`
- `background` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`
 
See an [example](example-tank).

### depth
The `depth` key corresponds to a non-negative value that specifies the depth of the water surface. If this key is excluded the depth defalts to zero, placing the surface of the water at the top of the tank and allowing fish to swim anywhere. If a positive value is specified than the surface of the water will be placed `depth` lines down, leaving `depth` lines of "air" at the top of the tank where fish cannot swim. Ducks swim at the surface of the water, so it is important to give them space for where their heads peak above the water.

### foreground & background
The animation and flipped_animation [animations](animations) should contain identically sized frames, but these animations NEED NOT have the same number of frames.

## Adding Fish

Fish are added to the tank using the `-f` flag followed by any number of fish names. The name of a fish may be used multiple times to add multiple of that fish to the tank. Those fish specified first will be rendered in front of those listed later. This flag is optional, but who wants a tank with no fish?
```
-f <fish_0> ... <fish_n>
```
Fish asset files are stored in  `~/.config/freefish/fish` and are `.json` files. These json files should contain the following key structure.
- `animation` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`
- `flipped_animation` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`
     
See an [example](example-fish).
 
### animation & flipped_animation

The animation and flipped_animation [animations](animations) should contain an identical number of identically sized frames.

## Adding Ducks
Ducks are added to the tank using the `-d` flag followed by any number of duck names. The name of a duck may be used multiple times to add multiple of that duck to the tank. Those ducks specified first will be rendered in front of those listed later. This flag is optional, but it shouldn't be.
```
-d <duck_0> ... <duck_n>
```
Duck asset files are stored in  `~/.config/freefish/fish` and are `.json` files. These json files should contain the following key structure.

- `buoyancy` (optional: defaults to 0)
- `animation` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`
- `flipped_animation` (see [Animations](animations))
    - `symbols`
    - `colors`
    - `highlights`

See an [example](example-duck).

### buoyancy

The `buoyancy` key corresponds to a value that specifies the number of lines of the duck that should appear above the surface of the water. If this key is excluded the buoyancy defoults to 0, so the top of the duck will be at the top layer of water. 

### animation & flipped animation

The animation and flipped_animation [animations](animations) should contain an identical number of identically sized frames.

## Adjusting Frame Speed

The delay between frames can be modified using the `-s` flag followed by the desired delay between frames in ms. The default delay is 200ms.
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

See [examples](examples).

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
    "animation": 
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
    "animation": {
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
## example tank
```
{
    "depth": 2,   <--- top 2 rows have no water
    "foreground": 
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
    "background": 
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

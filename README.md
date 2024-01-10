# rustyfish
A free fish tank for your terminal! An improved and expanded version of freefish, and a good way for me to get started with Rust.
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
* Clone this repository
```
$ git clone https://github.com/daviddwk/freefish.git
$ cd freefish
```
* Use cargo to build freefish
```
$ cargo build
```
* Initialize freefish 
run the binary with the init from from the cloned directory
```
$ ./target/debug/freefish -i
```
this creates the following folders, and populated them with the assents provided in the config folder
- ~/.config/tanks
- ~/.config/fish
- ~/.config/ducks

* Test that freefish was initialized properly by using the list argument, which lists available assets
```
$ ./target/debug/freefish -l
```
the following should be listed if freefish was initialized properly, possibly with even more assets
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

# Usage
Freefish is used by specifiying a tank and filling it with various fish and ducks. This is done by using the corresponding tank, fish, and duck flags and specifiying available assets to be used. Available assets can be listed by using the -l flag when running freefish.
```
$ ./freefish -t aquarium -f guppy clown guppy guppy angel -d duck
```

## Quitting

freefish can be stopped by pressing the q or Esc keys

## List

Available assets are listed using the -l flag. These assets can be used to selet and populate your own fish. When specifiying assets (tanks, fish, ducks) one should use the name of the asset excluding the .json extention.
```
-l
```

## Speed

The delay between frames can be modified using the -s flag. Following this flag the derired delay between frames is specified in ms.
```
-s <delay_ms>
```

## Tank
A tank is specified with the -t flag followed by the name of a single tank. Available tanks can be listed using -l.
```
-t <tank>
```
Tank asset files should be stored in ~/.config/tanks with the .json extention. They can then be utilzed using their name, excluding the .json extintion. These files should contain the following keys.

- "depth" (defaults to 0)
- "foreground" (see Animation)
    - "symbols"
    - "colors"
    - "highlights"
- "background" (see Animation)
    - "symbols"
    - "colors"
    - "highlights"

### depth
The depth key corresponds to a natural number that specifies the depth of the water. If this key is excluded the depth defalts to zero, placing the surface of the water at the top of the tank and allowing fish to swim anywhere. If a positive value, n, is specified than the surface of the water will be placed n lines down, leaving n lines of air at the top of the tank where fish cannot swim. Ducks swim at the surface of the water, so it is important to give them space where their heads peak above the water.

### foreground & background
The foreground and background should both be animations of the same size, but they NEED NOT have the same numbers of frames. 

## Fish

Fish are added to the tank using the -f flag followed by any number of fish to be added. The name of a fish may be used multiple times to add multiple of that fish to the tank. Those fish listed first will be rendered in front of those listed later. This flag is optional, but who wants a tank with no fish?
```
-f <fish_0> ... <fish_n>
```

## Ducks
Adding ducks to the tank works similarly as fish, using the -d flag followed by any number of ducks. These ducks will swim back and forth across the top of the water level, specificed by the tank's depth flag.
```
-d <duck_0> ... <duck_n>
```
Duck asset files should be stored as json files in ~/.config/ducks, where than can then be specified using their name excluding the .json extention. These files should contain the following keys.

- "buoyancy" (defaults to 0)
- "animation" (see Animation)
    - "symbols"
    - "colors"
    - "highligts"
- "flipped_animation" (see Animation)
    - "symbols"
    - "colors"
    - "highlights"

### buoyancy

The buoyancy key corresponds to a natural number that specifies the number of lines of the duck that should appear above the surface of the water. If this key is not specified it defaults to 0, where the top of the duck will be at the top layer of water. 

### animation & flipped animation

# Animations
Each sort of asset (tanks fish, and ducks) comtains animations in their json file, which consists of a name and the following subkeys.

- animation_name
    - "symbols"
    - "colors"
    - "highlights"

Each of the subkeys should contain a list of frames, which is expressed as a list of list of strings. Each frame must be the same size, thus contaning the same number of strings where each string is the same length.

Frames are broken up into thee speerate parts, each expressed as an equal size matrix of characters. These parts, symbols, colors and highlihts, and explained below.

### symbols

This portion contains the characters that make up the ascii art of the animation. Any space in this portion will be transparent, and the background will be rendered in its place.

### colors & highlights

Both the colors and highlihts sections contain charcters that will translate to the color of the charcters and their highlights specified under the symbols key. Each color character corresponds to the symbol located in the same position as the symbols matrix. The following color charcters can be used to color and highlight symbols with the terminals color palette.

- 'a' : DarkGrey
- 'r' : Red
- 'g' : Green
- 'y' : Yellow
- 'b' : Blue
- 'm' : Magenta
- 'c' : Cyan
- 'w' : White

- 'A' : Black
- 'R' : DarkRed
- 'G' : DarkGreen
- 'Y' : DarkYellow
- 'B' : DarkBlue
- 'M' : DarkMagenta
- 'C' : DarkCyan
- 'W' : Grey

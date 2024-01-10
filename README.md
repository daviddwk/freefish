# rustyfish
A free fish tank for your terminal! An improved and expanded version of freefish, and a good way for me to get started with Rust.

# Installation
* Clone this repository

```
git clone https://github.com/daviddwk/freefish.git
cd freefish
```

* Use cargo to build freefish

```
cargo build

```
* Initialize freefish 
run the binary with the init from from the cloned directory

```
./target/debug/freefish -i
```
this creates the following folders, and populated them with the assents provided in the config folder
- ~/.config/tanks
- ~/.config/fish
- ~/.config/ducks

* Test that freefish was initialized properly by using the list argument, which lists available assets
```
./target/debug/freefish -l
```
# Usage

Freefish is used by specifiying a tank and filling it with various fish and ducks. This is done by using the corresponding tank, fish, and duck flags and specifiying available assets to be used. Available assets can be listed by using the -l flag when running freefish.
```
./freefish -t aquarium -f guppy clown guppy guppy angel -d duck
```

## Tank
A single tank needs to be specified for freefish to display. This is done with the -t flag, following which a single argument should be suplied being the name of the desired tank. This tank may then be populated with fish and ducks.

A dank also has a depth attribute that describes how deep the water level is for said tank. If this field is left bland then then the whole tank may be occupied by swimming fish. 

Tank asset files should be stored in ~/.config/tanks with the .json extention. They can then be utilzed using their name, excluding the .json extintion. These files should contain the following keys.

- depth (defaults to 0)
- foreground (see Animation)
    - symbols
    - colors
    - highlights
- background (see Animation)
    - symbols
    - colors
    - highlights

### depth

### foreground & background

The foreground and background should both be animations of the same size, but they NEED NOT have the same numbers of frames. 

## Fish
A tank may be populated by fish that will swim around the available area. Using the -f flag, one should follow this flag with the desired fish to populate the tank. An argument for the same fish may be supplied multiple times to add multiple of that fish. Those fish listed first will be rendered in front of those listed later.

## Ducks
Adding ducks to the tank works similarly as fish. Using the -d flag, one can supply a number of ducks to the tank. These ducks will swim back and forth across the top of the water level, specificed by the tank's depth flag.

# Animations
Each sort of asset (tanks fish, and ducks) comtains animations in their json file, which consists of a name and the following subkeys.

- animation
    - symbols
    - colors
    - highlights

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

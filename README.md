# Rusty Pong

A two-player version of the classic game Pong, written in Rust using the [Tetra](https://tetra.seventeencups.net/) framework.

## Installation

_Requirements: macOS (pre-M1), linux._

macOS -> unzip `pong_release.zip` and follow the instructions in `install_readme.txt`

linux -> requires compilation via _cargo_ (sorry!):

1. install _cargo_ (comes packaged with Rust): https://www.rust-lang.org/tools/install. Note: make sure your PATH is updated if not done automatically
2. follow the instructions in `install_readme.txt` to install the [SDL 2.0](https://www.libsdl.org/index.php) dependency
3. run `./release.sh` which will build and package a release version as `pong_release.zip`
4. unzip `pong_release.zip` and play by running `./pong`

## Controls

_Move paddle up-down_ -> Player 1 "W" - "S", Player 2 "UP arrow", "DOWN arrow"  
_Exit_ -> Escape

## Tips

- try winning without letting your opponent score a single point!
- try changing settings for paddle and ball spin and acceleration in `settings.rs` to give the game a different feel!

## Resources

- built using the [Tetra](https://tetra.seventeencups.net/) 2D game framework
- assets (textures, sound effects) kindly provided for free by https://www.kenney.nl/

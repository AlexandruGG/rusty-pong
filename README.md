# Rusty Pong

A two-player version of the classic game Pong, written in Rust using the [Tetra](https://tetra.seventeencups.net/) framework.

## Installation

_Requirements: macOS, linux._

macOS -> unzip `pong_release.zip` and follow the instructions in `install_readme.txt`

linux -> requires compilation via _cargo_ (sorry!):

1. install _cargo_ (comes packaged with Rust): `curl https://sh.rustup.rs -sSf | sh`
2. run `./release.sh` which will build and package a release version as `pong_release.zip`
3. unzip `pong_release.zip` and follow the instructions in `install_readme.txt`

## Controls

_Move paddle up-down_ -> Player 1 "W" - "S", Player 2 "UP arrow", "DOWN arrow"  
_Exit_ -> Escape

## Tips

- try winning without letting your opponent score a single point!
- try changing settings for paddle and ball spin and acceleration in `settings.rs` to give the game a different feel!

## Resources

- built using the [Tetra](https://tetra.seventeencups.net/) 2D game framework
- assets (textures, sound effects) kindly provided for free by https://www.kenney.nl/

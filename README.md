# Single Hour Projects

A collection of projects that I completed in one hour.

## brainfuck 
A simple brainfuck interpreter written in Rust.

Usage:

`brainfuck <filename>`

## mine_sweeper
Mine Sweeper written in Rust

To controll the field size and mine count you have to 
edit some constants and recompile the project.

### How to play
Since this project had to be completed in one hour 
are the control elements, let's call it special...

To place a flag on field A4 you type `pa4`
To turn field F0 you type `tf0`

If you try to access a field that does not exist, the game may crash.

### Constants

`FIELD_WIDTH` Can not be bigger than 26

`FIELD_HEIGHT` Can not be bigger than 10

`BOMB_COUNT` Can not be bigger than 255
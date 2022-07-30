# Single Hour Projects

A collection of projects that I completed in one hour.

## brainfuck 
A simple brainfuck interpreter written in Rust.

Usage:

`brainfuck <filename>`

## mine_sweeper
Mine Sweeper written in Rust

To control the field size and mine count you have to 
edit some constants and recompile the project.

### How to play
Since this project had to be completed in one hour 
the controls are... Let's call it special.

To place a flag on field A4 you type `pa4`
To turn field F0 you type `tf0`

If you try to access a field that does not exist, the game will crash.

### Constants

`FIELD_WIDTH` Should not be bigger than 26

`FIELD_HEIGHT` Should not be bigger than 10

`BOMB_COUNT` Can not be bigger than 255

## radix_sort
Radix sort is a sorting algorithm without an comparisons. For small lists radix sort is relatively slow compared to more traditional algorithms.

## favpong
Pong in the fav icon of your browser.

# bf: Brainfuck

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/vtashkov/bf/actions/workflows/rust.yml/badge.svg)](https://github.com/vtashkov/bf/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/vtashkov/bf/graph/badge.svg?token=MWHQWW4AIB)](https://codecov.io/github/vtashkov/bf)

`bf` is a Brainfuck interpreter. It includes a library crate `bf` that exports most of the functionality,
and an executable `bf` that provides a command-line interface for executing 
Brainfuck programs.

## The language Brainfuck

The following is taken from the the wiki page of the language: [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. 
Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. 

The language consists of eight commands. A brainfuck program is a sequence of these commands, possibly interspersed with other characters (which are ignored). The commands are executed sequentially, with some exceptions: an instruction pointer begins at the first command, and each command it points to is executed, after which it normally moves forward to the next command. The program terminates when the instruction pointer moves past the last command.

The brainfuck language uses a simple machine model consisting of the program and instruction pointer, as well as a one-dimensional array of at least 30,000 byte cells initialized to zero; a movable data pointer (initialized to point to the leftmost byte of the array); and two streams of bytes for input and output (most often connected to a keyboard and a monitor respectively, and using the ASCII character encoding).

The eight language commands each consist of a single character:
| Character |	Meaning |
| --------- | ----------- |
| > | 	Increment the data pointer by one (to point to the next cell to the right) |
| < |	Decrement the data pointer by one (to point to the next cell to the left). |
| + |	Increment the byte at the data pointer by one. |
| - |	Decrement the byte at the data pointer by one. |
| . |	Output the byte at the data pointer. |
| , |	Accept one byte of input, storing its value in the byte at the data pointer. |
| [ |	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| ] |	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

~~[ and ] match as parentheses usually do: each [ matches exactly one ] and vice versa, the [ comes first, and there can be no unmatched [ or ] between the two.~~ This rule is removed, see below.

## Implementation specifics

- If a program attempts to move the pointer below the first array cell, or beyond the last array cell, then pointer will wrap around.
- The range of values of a single cell is from 0 to 255 (i.e. unsigned integer byte).
- If a program attempts to either decrement the value of a cell below its documented minimum value or increment the value of a cell beyond its documented maximum value, then the value in the cell after such an operation wraps around.
- If a program attempts to input a value when there is no more data in the input stream, the value in the current cell is unchanged.
- If a program contains one or more unbalanced `[` brackets, then the interpreter will assume that the corresponding `]` closing brackets are at the end (i.e. it will behave as if they were added). If a program contains a `]` closing bracket without corresponding `[` opening bracket, then the interpreter will ignore all instructions after that.

The goal is to minimize the errors that `bf` produces and try to be as forgiving as possible.

## Command line usage

```
Brainfuck interpreter

Usage: bf [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Path to the file to be interpreted

Options:
  -m, --memory-size <MEMORY_SIZE>  Number of the cells in the memory, defaults to 30 000 [default: 30000]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Library usage

This will output "Hello World!\n" in the output vector:

```
use std::io::Cursor;
use std::str;

use bf::Interpreter;

let source_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
let mut input = Cursor::new(vec![]);
let mut output = vec![];
let mut interpreter = Interpreter::new(&mut input, &mut output, 30000);
interpreter.execute(&source_code);
assert_eq!("Hello World!\n", str::from_utf8(output.as_slice()).unwrap());
```


## Examples

Example programs in Brainfuck language can be found in [/examples](https://github.com/vtashkov/bf/tree/master/examples) directory.

`hello_world` and `rot13` are taken from the wiki page of the language: [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

## About me

Written by Victor Tashkov <vtashkov@gmail.com>

Published with MIT License.

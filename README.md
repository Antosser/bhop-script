Bunny-hop script that presses the `;` at an arbitrary speed while the `space` key is held.

# Installation

To use this program, you will need to have Rust installed on your system. Once you have Rust installed, you can install the program by running the following command in
your terminal:

```
cargo install bhs
```

## Dependencies

This program uses the `inputbot` dependency, so make sure to install [it's dependencies](https://github.com/obv-mikhail/InputBot?tab=readme-ov-file#build-dependencies) before installing.

# Usage

When you run the program, it will prompt you to enter two values: the interval between checks (in milliseconds) and the hold period (also in milliseconds). Once you
have entered these values, the program will start monitoring the keyboard input for a space key and simulate the pressing of the semicolon key when it is pressed.

Here are some usage examples:

```
$ bhs
Enter interval between checks (ms): 15
Enter hold time (ms): 5
```

In this example, the program will check for a space key every 15 milliseconds and simulate the pressing of the semicolon key if it is pressed. The hold period is set to 5 milliseconds, so the program will simulate the pressing of the semicolon key and then release it after one second has passed.

# RP2040 contains a group of executables in src/bin which can be compiled and run on the  Pico W RP 2040 microcontroller.

This crate uses the embassy libraries examples/rp folder as a starting point.  However, the Cargo.toml file in that directory has many circular references to embassy-* crates with various patch levels that are difficult to modify.

We created this crate to simplify the Cargo.toml file and wherever possible, only use embassy-* versions that are avaiable as regular release verisons, avoiding patches and circular references to source code that is available on crates.io.

The cyw43 ethernet/bluetooth chip on the rp Pico presents special challenges.  The embassy examples are very helpful.  However, in order to better understand and build from the examples, we needed to simplify, understand, and then build upon them.

## Setup
* Target Pico W:  the target Pico W is connected to a Pico W bread board.  Power is applied to the target Pico W's micro usb adapter.

* A debug probe such as a pico-debug-probe or a waveshare RP2040 debug probe is connected via a USB cable from the computer to the Debug probe.

* The debug problem has a 3 wire connection from the debug probe's debug port to the debug port on the Target Pico W

* For "blink" scenarios where "21" is specified in the name of the source file, an LED is connected to pin 21 of the Pico W and a resister somewhere in teh range of 330 Ohms is connected from the other end of the LED  and to ground, setting up a voltage divider with the LED essentially acting as the top resister and the 330 ohm resistor acting as the bottom resistor.  Adding the resistor prevents the LED from experiencing the full 3.3 V drop and dissipates some of the power of the pull down.

## Running executables
To run the executables, run the following command:
cargo run <filename of rs file from src/bin without the '.rs' suffix>

e.g.
cargo run bink_21

The above comand will compile blink_21.rs and then load the compiled bare metal firmware onto the Target Pico W and start the Pico W.
Any debug statements will be printed back to stdio in the terminal that runs the cargo command.


## bink_21.rs
This file assumes an LED and a resistor (approximately 330 ohms) are connected to GPIO pin 21 on the Pico W.  The LED blinks on and off.

The file uses the embassy embedded async library to provide a simple thread for running the single thread which turns the LED on and off.  As this uses embassy, the polling is minimized and the CPU goes to sleep between each LED state change.

## blink_two_tasks_21
Creates two threads, each toggling the LED off of pin 21.  The tasks contain debug info which names the competing tasks which use a mutex to alternatively control the single LED off of pin 21.


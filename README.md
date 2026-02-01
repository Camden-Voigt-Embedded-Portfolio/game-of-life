# The Game of life on Microbit V2

Author: Camden Voigt

This is a simple project that runs Conway's game of life on the Microbit v2 led display. A random starting board will be chosen on program start and will play until the board is empty or it gets stuck in a loop. An A button press will generate new random boards while held and a B button press compliments the current board.

## Running

With probe-rs installed and a microbit v2 connected run to flash onto the microbit.

`cargo run --release`

## How it went

This project went pretty well. The display inteface made actually displaying the game of life quite simple. The difficulty came with the buttons. Figuring out the interface to interact with the buttons wasn't hard, but integrating them into a state machine was interesting. First, I tried have separate variables for the buttons pressed and the state. This worked fine, but felt messy. Next, I tried adding needed values to each State. So for instance the running state needs both button values to know how to get to the next state. So I put those values together in the Enum. This worked beautifully and is much cleaner.

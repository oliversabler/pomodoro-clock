# Pomodoro Clock
A very basic implementation of a pomodoro clock running in the terminal using [Rust](https://www.rust-lang.org/). This is not meant to be a serious project.

Alter global variables to set custom values, current default values are:
* `POMODORO`: 25 minutes
* `SHORT_BREAK`: 5 minutes
* `LONG_BREAK`: 15 minutes

## Ideas to implement
* GUI (?)
* Replace print line with permanent position of time, also show minutes if remaining time is more than a minute
* Replace constant values with arguments from terminal
* Wait for key input to start next iteration
* Do something idiotic like freeze the terminal when on a break

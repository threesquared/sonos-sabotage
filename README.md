# stuxsonos
State sponsored cyber warfare tool designed to target and disrupt Sonos networks.

## Description

> "It is the fight between tiger and elephant. If the tiger stands his ground, the elephant will crush him with its mass. But, if he conserves his mobility, he will finally vanquish the elephant, who bleeds from a multitude of cuts."
>
> -- <cite>Ho Chi Minh</cite>

This CLI tool contains a number of modes designed to take various covert actions to disrupt or disable sonos devices on the local network.

## Installation

TODO

## Usage

Get a list of options:

```sh
$ ./stuxsonos -h
```

### Modes

`-o, --oldMan`

This mode tracks the volume of all devices on the network. If it detects an increase of more than 2% it will reduce the volume of that 
device by 1.5x of the detected increase.

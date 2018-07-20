# stuxsonos [![GitHub release](https://img.shields.io/github/release/threesquared/stuxsonos.svg)](https://github.com/threesquared/stuxsonos/releases) [![Build Status](https://travis-ci.com/threesquared/stuxsonos.svg?branch=master)](https://travis-ci.com/threesquared/stuxsonos)

> "It is the fight between tiger and elephant. If the tiger stands his ground, the elephant will crush him with its mass. But, if he conserves his mobility, he will finally vanquish the elephant, who bleeds from a multitude of cuts."
>
> -- <cite>Ho Chi Minh</cite>

## Description

This CLI tool contains a number of modes designed to take various covert and overt actions to disrupt or disable sonos
devices on the local network for use in office cyber warfare.

## Installation

Download the latest binary for your platform from the [releases](https://github.com/threesquared/stuxsonos/releases) section.

## Usage

Get a full list of parameters:

```sh
$ ./stuxsonos -h
```

You can combine certain modes together and also set various options for others:

```sh
$ ./stuxsonos -oa -p "Beyonce" -i 1000
```

### Modes

`-o, --oldman`

This mode tracks the volume of all devices on the network. If it detects an increase of more than 5 points it will reduce the volume of that 
device by 1.3 times the detected increase in percentage points.

`-a, --assassin`

This mode watches the currently playing track and matches the artist or track name to a pattern. If a match is found the track
will be skipped. Pattern defaults to `Ed Sheeran` if not supplied.

`-d, --dictator`

This mode finds any devices not playing the supplied track and will clear the current queue and play the preferred track.
Track defaults to Guerrilla Radio by Rage Against the Machine if not supplied.

`-s, --saboteur`

This mode randomly performs operations like muting speakers, skipping/pausing tracks and changing the volume.

`-t, --totalitarian`

This mode will clear the queue of all devices and stop any running tracks it finds.

### Options

`-i <interval>`

The internal tick rate to poll devices and take actions in ms. Defaults to `10000`

`-p <pattern>`

The regex pattern to try and match against playing tracks and artists in assassin mode.
Defaults to `Ed Sheeran`

`-u <uri>`

The track uri to play in dictator mode.

`-x <ip>`

Only perform actions against a specific device IP address

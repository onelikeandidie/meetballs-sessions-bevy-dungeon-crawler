# A Meetballs Session on Bevy

_This session was made for Bevy `0.14.2`_

The goal is to create a simple retro turn-based 2d dungeon crawler with a
procedural generated dungeon using some kenney.nl assets.

This session is split into 2 sessions, Bevy is a game engine that requires a
lot of time to learn and the basics of bevy will take up 1 session on its own.
Also because I could not prepare everything in a digestible way.

The projects included are in the following order:

- [Basic Project](./basic_project)
  - Introduction to bevy, how to create your own components and systems to
  create a simple program
- [Refactored Project](./refactored_project)
  - How to refactor your game into a plugin and how to handle input
- [Asset Loading](./asset_loading)
  - How to load sprites and fonts, spawn sprites in your world, creating a
  camera, creating UI and handling pausing and unpausing the game

## Getting started

Follow the basic setup steps on the [Bevy getting started
page](https://bevyengine.org/learn/quick-start/getting-started/setup/). The
most important part of this is to enable the `dynamic_linking` feature of bevy,
this is literally a 5000000% decrease in subsequent builds. I cannot stress
this enough, this is crucial to not spend 2 hours debugging your game.




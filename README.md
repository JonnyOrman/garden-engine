# garden-engine

> Early stage WIP

A highly modular game engine written in Rust

Garden is an experimental, highly modular game engine. It consists of a range of small, granular libraries that act as building blocks to be put together to build an engine with the specific features you need for your game. You can construct an engine for anything from a 2D side scrolling game, a fully 3D game or a text based adventure game!

This allows you to start out with a simple foundation and then add only exactly what you need for your game. In its simplest form, a 'game' built with Garden might just build a program with an empty loop that doesn't really do anything, but then you can enrich it with components to add the functionality you need, such as graphics rendering, player input capture, physics, AI or anything else you might need.

It is highly extensible and you can create your own custom components.

Garden is written in Rust, providing safety and reliability.
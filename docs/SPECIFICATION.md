# Specification

Repository: https://github.com/IndaPlus22/bwidman-twhite-avidf-project

Project: https://github.com/orgs/IndaPlus22/projects/6

## Conventions

Commit messages will be in future tense, like:
> Fix this bug
Pull requests will be named after its associated issue, like:
> #13, Improve performance
For the Rust code, regular Rust naming conventions will be used according to its [style guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md) and [documentation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html).

## Description

The goal of the project is to create a fluid simulator. The programming language that will be used is Rust 🦀 along with the graphics library Piston. It is assumed that the entire screen space is filled with the fluid and boundary conditions will be set by walls along the window border. There will also be a GUI that enables the user to change fluid properties, e.g. viscosity, density, velocity diffusion perhaps vorticity. We will make a 2D simulator similar to one made by [Paveldogreat](https://paveldogreat.github.io/WebGL-Fluid-Simulation/) since it is feasible for us to create something of equivalent complexity. If time permits we will try to add certain features, e.g. addable objects as boundaries in the scene for the fluid to interact with and a scene that’s only partially filled with fluid. Time stamps are very difficult to predict for now since we’re uncertain about the specific milestones needed to complete the project.

## Work distribution

We have some initial ideas for work distribution but are flexible in how we approach it since it’s not clear if there are any distinct roles to be filled throughout the project. However, in the beginning Benjamin will set up the graphics environment and Tim and Avid will begin with the foundational math, partly by creating a function for solving systems of linear equations with the Gauss-Seidel method, which Benjamin also will join in to help with. Later, Benjamin will also implement mouse controls for interacting with the fluid.

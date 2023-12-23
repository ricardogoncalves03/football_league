# Football League CLI Program

This Rust project is a CLI program that simulates a football league where six teams play against each other. The program calculates the champion based on a point system (3 points for a win, 1 for a draw, and 0 for a loss).

## Project Structure
.
├── league.rs
├── main.rs
├── matches.rs
├── teams.rs
├── tests.rs
└── user_bet.rs

The project consists of the following components:

- league.rs: Contains logic for league management, standings, and match simulations.
- main.rs: Entry point for the Rust program.
- matches.rs: Logic related to matches and gameplay.
- teams.rs: Structures and functions related to teams participating in the league.
- tests.rs: Test suite for the program.
- user_bet.rs: Handles user betting logic and interaction.

## Goal

The primary goal of this project is to consolidate my learnings in Rust up to Chapter 12 of "The Rust Programming Language" book ([link to the book](https://rust-book.cs.brown.edu/)). Through this project, I aim to practice concepts like enums, structs, modules, HashMaps, and testing in Rust.

## Running the Program

Ensure you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository:

```bash
git clone <repository_url>
```

Navigate to the project directory and run:

```bash
cargo run
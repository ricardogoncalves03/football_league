# Football League CLI Program

This Rust project simulates a football league where six teams play against each other. The program calculates the champion based on a point system (3 points for a win, 1 for a draw, and 0 for a loss). The program offers interaction with the user via the command line interface (CLI). The user can engage by placing a bet on the team he believes will emerge as the league's winner. Input prompts guide the user through the selection process for his bet before the final standings are displayed.



## Project Structure

The project consists of the following components:

- `league.rs`: Contains logic for league management, standings, and match simulations.
- `main`: Executable file for the CLI program.
- `main.rs`: Entry point for the Rust program.
- `matches.rs`: Logic related to matches and gameplay.
- `teams.rs`: Structures and functions related to teams participating in the league.
- `tests.rs`: Test suite for the program.
- `user_bet.rs`: Handles user betting logic and interaction.

## Goal

The primary goal of this project is to consolidate my learnings in Rust up to Chapter 12 of "The Rust Programming Language" book ([link to the book](https://rust-book.cs.brown.edu/)). Through this project, I aim to practice concepts like enums, structs, modules, and testing in Rust.

## Running the Program

Ensure you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository:

```bash
git clone <repository_url>
```

Navigate to the project directory and run:

```bash
cargo run
```

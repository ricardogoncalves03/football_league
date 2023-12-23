use std::io;
use crate::teams::Team;

pub fn take_bet(teams: &[Team]) -> Option<usize> {
    println!("Place your bet on a team:");
    for (index, team) in teams.iter().enumerate() {
        println!("{} - {}", index + 1, team.name);
    }

    loop {
        println!("Enter the number of your chosen team:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input to get the team index
        let chosen_index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if chosen_index <= teams.len() && chosen_index > 0 {
            return Some(chosen_index - 1);
        } else {
            println!("Invalid team number. Try again");
        }
    }
}

pub fn check_bet_result(winner_index: usize, teams: &[Team]) {
    let max_score_team_index = teams.iter().enumerate().max_by_key(
        |(_, team)| team.score).map(|(index, _)| index
    );

    match max_score_team_index {
        Some(index) if index == winner_index => println!("Congratulations! You won!"),
        _ => println!("Better luck next time!"),
    }
}

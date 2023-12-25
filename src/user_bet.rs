use crate::teams::Team;
use std::io;

pub fn take_bet(teams: &[Team]) -> Option<usize> {
    println!("Place your bet on a team:");
    for (index, team) in teams.iter().enumerate() {
        println!("{} - {}", index + 1, team.name);
    }

    loop {
        println!("Enter the number of your chosen team:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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
    // Finding the index of the team with the maximum score in the league
    let max_score_team_index = teams
        .iter()
        .enumerate()
        .max_by_key(|(_, team)| team.score)
        .map(|(index, _)| index);

    // Comparing the user's bet index with the index of the team that won the league
    match max_score_team_index {
        Some(index) if index == winner_index => println!("Congratulations! You won!"),
        _ => println!("Better luck next time!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::teams::Team;

    // Test when the user's bet is on the winning team
    #[test]
    fn test_check_bet_result_winner() {
        let teams = vec![
            Team {
                name: "Team 1".to_string(),
                score: 3,
            },
            Team {
                name: "Team 2".to_string(),
                score: 5,
            },
        ];
        let winner_index: usize = 1; // Index of Team 2, the winning team
        let max_score_team_index = teams
            .iter()
            .enumerate()
            .max_by_key(|(_, team)| team.score)
            .map(|(index, _)| index);

        assert_eq!(max_score_team_index, Some(winner_index));
    }

    // Test when the user's bet is not on the winning team
    #[test]
    fn test_check_bet_result_loser() {
        let teams = vec![
            Team {
                name: "Team 1".to_string(),
                score: 3,
            },
            Team {
                name: "Team 2".to_string(),
                score: 5,
            },
        ];
        let loser_index = 1; // Index of Team 2, not the winning team
        let max_score_team_index = teams
            .iter()
            .enumerate()
            .max_by_key(|(_, team)| team.score)
            .map(|(index, _)| index);

        assert_ne!(max_score_team_index, Some(loser_index));
    }

    #[test]
    fn test_check_bet_result_draw() {
        let teams = vec![
            Team {
                name: "Team 1".to_string(),
                score: 5,
            },
            Team {
                name: "Team 2".to_string(),
                score: 5,
            },
        ];
        let bet_index = 0; // Index of Team 1, but it's a draw
        let max_score_team_index = teams
            .iter()
            .enumerate()
            .max_by_key(|(_, team)| team.score)
            .map(|(index, _)| index);

        assert_ne!(max_score_team_index, Some(bet_index));
    }

    #[test]
    fn test_check_bet_result_no_teams() {
        let teams: Vec<Team> = vec![];
        let max_score_team_index = teams
            .iter()
            .enumerate()
            .max_by_key(|(_, team)| team.score)
            .map(|(index, _)| index);

        assert!(max_score_team_index.is_none());
    }
}

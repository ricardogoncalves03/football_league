use crate::matches::play_match;
use crate::teams::Team;

fn get_num_teams(teams: &[Team]) -> usize {
    teams.len()
}

pub fn simulate_league(teams: &mut [Team]) -> Result<(), String> {
    if teams.len() % 2 != 0 {
        return Err("The league requires an even number of teams.".to_string());
    }
    // Iterate through each team
    for i in 0..get_num_teams(teams) {
        // For each team, iterate through remaining teams to avoid duplicate matches
        for j in (i + 1)..get_num_teams(teams) {
            // Clone the teams for simulation
            let mut team1 = teams[i].clone();
            let mut team2 = teams[j].clone();

            play_match(&mut team1, &mut team2);

            // Update the teams slice with the results
            teams[i] = team1;
            teams[j] = team2;
        }
    }

    Ok(())
}

pub fn display_standings(teams: &[Team]) -> String {
    let mut sorted_teams = teams.to_vec();
    sorted_teams.sort_by_key(|team| team.score);

    let mut standings = String::new();
    standings.push_str("\n-----------------\n");
    standings.push_str("League Standings:\n");
    standings.push_str("\n");

    for i in (0..get_num_teams(teams)).rev() {
        let team = &sorted_teams[i];
        standings.push_str(&format!("{} - {}: {} points\n", 
        get_num_teams(teams) - i, team.name, team.score));
    }
    standings.push_str("-----------------\n");

    standings
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::teams::Team;

    #[test]
    fn test_get_num_teams() {
        let teams = vec![Team::new("Team 1"), Team::new("Team 2")];
        assert_eq!(get_num_teams(&teams), 2);
    }

    #[test]
    fn test_standings_with_random_order_teams() {
        let teams = vec![
            Team { name: "Team A".to_string(), score: 10 },
            Team { name: "Team B".to_string(), score: 15 },
            Team { name: "Team C".to_string(), score: 5 },
        ];
        let standings = display_standings(&teams);

        let expected_output = "\
        \n-----------------\n\
        League Standings:\n\
        \n\
        1 - Team B: 15 points\n\
        2 - Team A: 10 points\n\
        3 - Team C: 5 points\n\
        -----------------\n";

        assert_eq!(standings, expected_output);
    }

}
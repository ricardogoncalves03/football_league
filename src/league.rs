use crate::matches::play_match;
use crate::teams::Team;

fn get_num_teams(teams: &[Team]) -> usize {
    teams.len()
}

pub fn simulate_league(teams: &mut [Team]) {
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
}

pub fn display_standings(teams: &[Team]) {
    let mut sorted_teams = teams.to_vec();
    sorted_teams.sort_by_key(|team| team.score);

    println!(" ");
    println!("-----------------");
    println!("League Standings:");
    println!(" ");

    for i in (0..get_num_teams(teams)).rev() {
        let team = &sorted_teams[i];
        println!("{} - {}: {} points", get_num_teams(teams) - i, team.name, team.score)
    }
    println!("-----------------");
}

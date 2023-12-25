use football_league::league::{display_standings, simulate_league};
use football_league::teams::Team;

#[test]
fn test_league_simulation_and_standings() {
    // Create a list of teams for the league
    let mut teams = vec![
        Team::new("Team 1"),
        Team::new("Team 2"),
        Team::new("Team 3"),
        Team::new("Team 4"),
    ];

    // Simulate the league
    let simulation_result = simulate_league(&mut teams);
    assert!(simulation_result.is_ok(), "League simulation failed");

    // Get the standings
    let standings = display_standings(&teams);
    assert!(!standings.is_empty(), "Standings should not be empty");

    // Check if the standings string format is as expected
    assert!(
        standings.contains("League Standings:"),
        "Standings string format is not as expected"
    );

    // Check for the presence of team names in the standings string
    for team in &teams {
        assert!(
            standings.contains(&team.name),
            "Standings do not contain the team name: {}",
            team.name
        );
    }

    // Example: Check if the scores are present in the standings string
    // This is a basic check and assumes that scores are part of the standings string
    for team in &teams {
        assert!(
            standings.contains(&team.score.to_string()),
            "Standings do not contain the score for team: {}",
            team.name
        );
    }
}

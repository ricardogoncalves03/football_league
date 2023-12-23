// matches.rs could have a Match struct.
// Employ a Vector in the matches module to track played matches
use crate::teams::Team;

use rand::random;

#[derive(Debug)]
pub enum MatchResult {
    Win,
    Draw, 
    Lose,
}

pub fn play_match(team1: &mut Team, team2: &mut Team) -> MatchResult {
    let match_outcome = random::<f32>();
    println!("{match_outcome}");

    if match_outcome > 0.66 {
        // Team 1 wins
        team1.update__score(3);
        MatchResult::Win
    } else if match_outcome < 0.33 {
        // Team 2 wins
        team2.update__score(3);
        MatchResult::Lose
    } else {
        // Draw
        team1.update__score(1);
        team2.update__score(1);
        MatchResult::Draw
    }
}

// matches.rs could have a Match struct.
// Employ a Vector in the matches module to track played matches
use rand::random;

pub enum MatchResult {
    Win,
    Draw, 
    Lose,
}

fn play_match(team1: &Team, team2: &Team) {
    let match_outcome = random::<f32>();

    if match_outcome > 0.5 {
        team1.update__score(3);
        MatchResult::Win
    }
}
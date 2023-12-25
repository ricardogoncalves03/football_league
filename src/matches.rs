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

    if match_outcome > 0.66 {
        // Team 1 wins
        team1.update_score(3);
        MatchResult::Win
    } else if match_outcome < 0.33 {
        // Team 2 wins
        team2.update_score(3);
        MatchResult::Lose
    } else {
        // Draw
        team1.update_score(1);
        team2.update_score(1);
        MatchResult::Draw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teams::Team;

    #[test]
    fn test_play_match() {
        let mut team1 = Team::new("Team 1");
        let mut team2 = Team::new("Team 2");
        let result = play_match(&mut team1, &mut team2);
        // Assert based on the result and updated scores.
    }

    // Additional tests simulating different match outcomes.
}

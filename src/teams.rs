#[derive(Debug, Default, Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            score: 0,
        }
    }

    pub fn update_score(&mut self, points: u32) {
        self.score += points;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_team() {
        let team = Team::new("Test Team");
        assert_eq!(team.name, "Test Team");
        assert_eq!(team.score, 0);
    }

    #[test]
    fn test_update_score() {
        let mut team = Team::new("Test Team");
        team.update_score(5);
        assert_eq!(team.score, 5);
    }
}

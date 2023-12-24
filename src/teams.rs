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

    pub fn update_score(&mut self, points:u32) {
        self.score += points;
    }
}

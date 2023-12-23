// teams.rs might have a Team struct.

// Use a HashMap in the teams module to store teams' data.

#[derive(Debug, Default, Clone)]
pub struct Team {
    pub name: String,
    _country: String,
    pub score: u32,
    // Add new fields here accordingly
}

impl Team {
    pub fn new(name: &str, _country: &str) -> Self {
        Team {
            name: name.to_string(),
            _country: _country.to_string(),
            score: 0,
        }
    }

    pub fn update_score(&mut self, points:u32) {
        self.score += points;
    }
/* 
    pub fn get_score(&self) -> u32 {
        self.score
    }
*/
}

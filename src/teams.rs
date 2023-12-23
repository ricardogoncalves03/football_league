// teams.rs might have a Team struct.

// Use a HashMap in the teams module to store teams' data.

#[derive(Debug)]
pub struct Team {
    _name: String,
    _country: String,
    _score: u32,
    // Add new fields here accordingly
}

impl Team {
    pub fn new(_name: &str, _country: &str) -> Self {
        Team {
            _name: _name.to_string(),
            _country: _country.to_string(),
            _score: 0,
        }
    }
    pub fn update__score(&mut self, points:u32) {
        self._score += points;
    }
}

// teams.rs might have a Team struct.

// Use a HashMap in the teams module to store teams' data.

#[derive(Debug)]
pub enum Team {
    Barcelona,
    RealMadrid,
    BayernMunich,
    ManCity,
    Liverpool,
    Benfica,
}

pub struct TeamInfo {
    name: String,
    country: String,
    //TO DO - add new fields
}

impl Team {
    pub fn details(&self) -> TeamInfo {
        match self {
            Team::Barcelona => TeamInfo {
                name: "Barcelona".to_string(),
                country: "Spain".to_string(),
            },
            Team::RealMadrid => TeamInfo {
                name: "Real Madrid".to_string(),
                country: "Spain".to_string(),
            },
            Team::BayernMunich => TeamInfo {
                name: "Bayern Munich".to_string(),
                country: "Germany".to_string(),
            }, //TO DO - add missing clubs
            _ => TeamInfo {
                name: "Unknown".to_string(),
                country: "Unknown".to_string(),
            }
        }
    }
}

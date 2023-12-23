mod teams;
mod matches;
mod league;

use teams::Team;
use league::{display_standings, simulate_league};

fn main() {
    let barcelona = Team::new("Barcelona", "Spain");
    let real_madrid = Team::new("Real Madrid", "Spain");
    let bayern_munich = Team::new("Bayern Munich", "Germany");
    let manchester_city = Team::new("Manchester City", "England");
    let mut teams = vec![barcelona, real_madrid, bayern_munich, manchester_city];

    // Each team play agains each other 2 times
    simulate_league(&mut teams);
    simulate_league(&mut teams);
    // Display final results
    display_standings(&teams);
}

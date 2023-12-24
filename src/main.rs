mod teams;
mod matches;
mod league;
mod user_bet;

use teams::Team;
use league::{display_standings, simulate_league};
use user_bet::{take_bet,  check_bet_result};

fn main() {
    let barcelona = Team::new("Barcelona");
    let real_madrid = Team::new("Real Madrid");
    let bayern_munich = Team::new("Bayern Munich");
    let manchester_city = Team::new("Manchester City");
    let mut teams = vec![barcelona, real_madrid, bayern_munich, manchester_city];

    // Each team plays against each other 2 times
    simulate_league(&mut teams);
    simulate_league(&mut teams);

    // User places a bet
    if let Some(selected_index) = take_bet(&teams) {
        display_standings(&teams);
        check_bet_result(selected_index, &teams);
        // Display final results
    } else {
        println!("Invalid selection or input");
    }
}

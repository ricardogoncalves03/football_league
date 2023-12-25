mod league;
mod matches;
mod teams;
mod user_bet;

use league::{display_standings, simulate_league};
use teams::Team;
use user_bet::{check_bet_result, take_bet};

fn main() {
    let barcelona = Team::new("Barcelona");
    let real_madrid = Team::new("Real Madrid");
    let bayern_munich = Team::new("Bayern Munich");
    let manchester_city = Team::new("Manchester City");
    let mut teams = vec![barcelona, real_madrid, bayern_munich, manchester_city];

    // Each team plays against each other 2 times
    match simulate_league(&mut teams) {
        Ok(_) => {
            println!("League simulation completed successfully.");

            // User places a bet
            if let Some(selected_index) = take_bet(&teams) {
                display_standings(&teams);
                check_bet_result(selected_index, &teams);
                // Display final results
            } else {
                println!("Invalid selection or input");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

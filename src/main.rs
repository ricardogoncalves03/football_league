mod teams;
mod matches;

use teams::Team;
use matches::play_match;

fn main() {
    // Access team details
    let barcelona = &mut Team::new("Barcelona", "Spain");
    // println!("{:?}", barcelona);
    let real_madrid = &mut Team::new("Real Madrid", "Spain");

    let result = play_match(barcelona, real_madrid);
    println!("{:?}", result);
}

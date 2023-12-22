mod teams;

use teams::{Team, TeamInfo};

fn main() {
    // Access team details
    let barcelona = Team::Barcelona;
    let barcelona_details = barcelona.details();
    println!("{:?}", barcelona_details);
}

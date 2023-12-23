mod teams;

use teams::Team;

fn main() {
    // Access team details
    let barcelona = Team::new("Barcelona", "Spain");
    println!("{:?}", barcelona);
}

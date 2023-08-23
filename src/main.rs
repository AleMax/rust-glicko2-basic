mod glicko2;

fn main() {

    //let mut player: glicko2::Glicko2Instance = glicko2::Glicko2Instance::from_values_elo(1500., 200., 0.06);

    let mut player: glicko2::Glicko2Instance = glicko2::Glicko2Instance::from_values_elo(2300., 120., 0.06);

    let enemy1: glicko2::Glicko2Instance = glicko2::Glicko2Instance::from_values_elo(1500., 350., 0.06);


    let enemy2: glicko2::Glicko2Instance = glicko2::Glicko2Instance::from_values_elo(1550., 100., 0.06);
    let enemy3: glicko2::Glicko2Instance = glicko2::Glicko2Instance::from_values_elo(1700., 300., 0.06);

    //player.add_opponent((enemy1, 1.0));
    let winning_probability = player.get_winning_probability(enemy1);
    //player.add_opponent((enemy2, 0.0));
    //player.add_opponent((enemy3, 0.0));

    println!("Win Probability: {:?}", winning_probability);

    //player.compute_rating();


    //println!("New Rating: {:?}", player.get_rating());
    
}

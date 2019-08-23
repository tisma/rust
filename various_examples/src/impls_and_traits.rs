struct Player {
	first_name: String,
	last_name: String,
}

trait FullName {

	fn new(first_name: String, last_name: String) -> Player;

    fn full_name(&self) -> String;
}

trait Person {
    fn full_name(&self) -> String;
}

trait Employee : Person {
    fn job_title(&self) -> String;
}

trait ExpatEmployee : Employee + FullName {
    fn additional_tax(&self) -> f64;
}

impl FullName for Player {

	fn new(first_name: String, last_name: String) -> Player {
	    Player {
	    	first_name: first_name,
	    	last_name: last_name,
	    }
	}

	fn full_name(&self) -> String {
	    format!("{} {}", self.first_name, self.last_name)
	}
}

fn main() {
    let player1 = Player {
    	first_name: "Rafael".to_string(),
    	last_name: "Nadal".to_string(),
    };

    println!("Player 1: {}", player1.full_name());

    let player_name = Player::new("Serena".to_string(), "Williams".to_string()).full_name();
    println!("Player 2: {}", player_name);
}

struct Player {
    name: String,
    iq: u8,
    friends: u8,
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100,
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, number: u8) {
        self.friends = number;
    }
}

fn main() {
    println!("Hello, world!");
    let mut player = Player::with_name("Tom");
    player.set_friends(9);
    println!(
        "{}'s IQ: {} | friends count: {}",
        player.name,
        player.iq,
        player.get_friends()
    );
    // let _ = Player::get_friends(&player);
}

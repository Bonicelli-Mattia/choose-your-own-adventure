use std::io;
use rand::Rng;

#[derive(PartialEq)]
enum Command {
    Go(Direction),
    Unlock(Direction),
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Exit {
    direction: Direction,
    target: usize,
    locked: bool,
}

impl Exit {
    fn can_go(&self, direction: &Direction) -> bool {
        self.direction == *direction &&
        !self.locked
    }
}

struct Room {
    description: String,
    exits: Vec<Exit>,
}

impl Room {
    fn unlock(&mut self, direction: Direction) -> Option<usize> {
        let exit = self.exits.iter_mut()
                             .find(|e| e.direction == direction)
                             .unwrap();

        exit.locked = false;

        None
    }

    fn can_go(&self, direction: Direction) -> bool {
        self.exits.iter().find(|e| e.can_go(&direction)).is_some()
    }

    fn exit_to(&self, direction: Direction) -> Option<usize> {
        Some(self.exits.iter()
                  .find(|e| e.direction == direction)
                  .unwrap()
                  .target
            )
    }

    fn is_escape(&self) -> bool {
        self.exits.len() == 0
    }

    fn is_locked(&self, _direction: Direction) -> bool {
        self.exits.iter().find(|e| e.locked).is_some()
    }
}

fn rps() {

    println!("The exit is guarded by an massive orc, he challenges you to rock, paper, scissors\n");
    
    loop {
        println!("Choose your hand\n");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed");
        let guess = guess.trim();
        

        let computer_choice = rand::thread_rng().gen_range(1, 4);

        if computer_choice == 1 && guess == "rock" {
            println!("computer chooses rock its a tie!\n");
        } else if computer_choice == 1 && guess == "paper" {
            println!("computer chooses rock you WIN!!!\n");
            break;
        } else if computer_choice == 1 && guess == "scissors" {
            println!("computer chooses rock You LOSE!!!\n");
        } else if computer_choice == 2 && guess == "rock" {
            println!("computer chooses paper you LOSE!!!\n");
        } else if computer_choice == 2 && guess == "paper" {
            println!("computer chooses paper a tie\n");
        } else if computer_choice == 2 && guess == "scissors" {
            println!("computer chooses paper you WIN!!!\n");
            break;
        } else if computer_choice == 3 && guess == "rock" {
            println!("computer chooses scissors you WIN!!!\n");
            break;
        } else if computer_choice == 3 && guess == "paper" {
            println!("computer chooses scissors you LOSE!!!\n");
        } else if computer_choice == 3 && guess == "scissors" {
            println!("computer chooses scissors a tie!\n")
        }
    }
}

fn main() {

    use Direction::South as south;
    use Direction::East as east;
    use Direction::West as west;
    use Direction::North as north;

    let mut rooms = vec![
        Room {
            description: "You find yourself in a dark dungeon, you need to escape. There is a door to the south and a door to the east.".to_string(),
            exits: vec![
                Exit {
                    direction: south,
                    target: 2,
                    locked: false,
                },
                Exit {
                    direction: east,
                    target: 1,
                    locked: false,
                },
            ],
        },
        Room {
            description: "You find yourself in a room. There is a door to the west and a door to the south.".to_string(),
            exits: vec![
                Exit {
                    direction: west,
                    target: 0,
                    locked: false,
                },
                Exit {
                    direction: south,
                    target: 3,
                    locked: false,
                },
            ],
        },
        Room {
            description: "You find yourself in a room. There is a door to the north.".to_string(),
            exits: vec![
                Exit {
                    direction: north,
                    target: 0,
                    locked: false,
                },
            ],
        },
        Room {
            description: "You find yourself in a room. There is a door to the north. The door to the south is locked.".to_string(),
            exits: vec![
                Exit {
                    direction: north,
                    target: 1,
                    locked: false,
                },
                Exit {
                    direction: south,
                    target: 4,
                    locked: true,
                },
            ],
        },
        Room {
            description: "Dungeon exit".to_string(),
            exits: vec![],
        }
    ];

    let mut current_room = 0;

    println!("\n\n");
    println!("######## CHOOSE YOUR OWN RUST ########\n\n");

    while !rooms[current_room].is_escape() {
        current_room = enter(rooms.get_mut(current_room).unwrap()).unwrap_or(current_room);
    }

    rps();

    println!("######## YOU HAVE ESCAPED ########");
}

fn enter(room: &mut Room) -> Option<usize> {

    use Direction::South as south;
    use Direction::East as east;
    use Direction::West as west;
    use Direction::North as north;

    let go = Command::Go;
    let unlock = Command::Unlock;

    let mut command: Option<Command> = None;

    while command == None {
        println!("{}", room.description);
        println!("\nWhat do you do?\n");

        for exit in room.exits.iter() {
            match exit.direction {
                north => println!("* Go (n)orth"),
                east  => println!("* Go (e)ast"),
                south => println!("* Go (s)outh"),
                west  => println!("* Go (w)est"),
            }

            if exit.locked {
                match exit.direction {
                    north => println!("* (un) unlock north"),
                    east  => println!("* (ue) unlock east"),
                    south => println!("* (us) unlock south"),
                    west  => println!("* (uw) unlock west"),
                }
            }
        }

        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        command = match input.trim() {
            "n" if room.can_go(north) => Some(go(north)),
            "e" if room.can_go(east)  => Some(go(east)),
            "s" if room.can_go(south) => Some(go(south)),
            "w" if room.can_go(west)  => Some(go(west)),

            "un" if room.is_locked(north) => Some(unlock(north)),
            "ue" if room.is_locked(east)  => Some(unlock(east)),
            "us" if room.is_locked(south) => Some(unlock(south)),
            "uw" if room.is_locked(west)  => Some(unlock(west)),

            _   => {
                println!("Please type a valid command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        Command::Go(north) => room.exit_to(north),
        Command::Go(east)  => room.exit_to(east),
        Command::Go(south) => room.exit_to(south),
        Command::Go(west)  => room.exit_to(west),
        Command::Unlock(d) => room.unlock(d),
    }
}

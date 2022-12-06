// Feeling things out
// enum with variant values for class/race... make a impl method based on sub class/race?
// use the match enum binding... somewhere. Maybe with the read_lines return?
use std::io;

struct Character {
    name: String,
    class: String,
    race: String,
    level: i32,
    age: i32,
    size: String,
    height: String,
    weight: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}


impl Default for Character {
    fn default() -> Self {
        Character {
		name: String::from(""),
		class: String::from(""),
		race: String::from(""),
		level: 0,
		age: 0,
		size: String::from(""),
		height: String::from(""),
		weight: 0,
		strength: 0,
		dexterity: 0,
		constitution: 0,
		intelligence: 0,
		wisdom: 0,
		charisma: 0,
        }
    }
}


fn main() {
    // Create vec of Characters
    let mut characters: Vec<Character> = Vec::new();

    // Loop Character Creation
    loop {
        let mut character = Character::default();

    //Read user input for the Name field
        println!("Please enter your Character's name:");
        std::io::stdin().read_line(&mut character.name).expect("Failed to read input");

    // Add Character to Character Vec
    characters.push(character);

    // Give option to create more
    println!("Create another Character? (y/n)");
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).expect("Failed to read input");
    if answer.trim() == "n" {
        break;
    }

    }

}

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

// fn parse_args() -> Result<Character, String> {
//     let mut character = Character::default;
//     let mut fields = vec![
//         "Name",
//         "Class",
//         "Race",
//         "Level",
//         "Age",
//         "Size",
//         "Height",
//         "Weight",
//         "Strength",
//         "Dexterity",
//         "Constitution",
//         "Intelligence",
//         "Wisdom",
//         "Charisma"];

//     loop {
//         println!("Please enter your Character's Stats:");
//         for field in fields {
//             println!("{}:",  field);
//             let value = read!("{}\n");
//             match field {
//                 "Name" => character.name = value,
//                 "Class" => character.class = value,
//                 "Race" => character.race = value,
//                 "Level" => character.level = value,
//                 "Age" => character.age = value,
//                 "Size" => character.size = value,
//                 "Height" => character.height = value,
//                 "Weight" => character.weight = value,
//                 "Strength" => character.strength = value,
//                 "Dexterity" => character.dexterity = value,
//                 "Constitution" => character.constitution = value,
//                 "Intelligence" => character.intelligence = value,
//                 "Wisdom" => character.wisdom = value,
//                 "Charisma" => character.charisma = value,
//                 _ => (),
//             }
//             fields.remove(field);
//         }
//         if fields.is_empty() {
//             break;
//         }
//     }
//     Ok(character)
// }


fn main() {
}

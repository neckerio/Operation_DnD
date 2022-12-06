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
    // Create a Vec to store Characters
    let mut creations: Vec<Character> = Vec::new();

    // Loop until the user enters "Done"
    loop {
	let mut character = Character::default();

	// Create a Vec of field names that have not been set
	let mut unset_fields = vec![
	    "name",
	    "class",
	    "race",
	    "level",
	    "age",
	    "size",
	    "height",
	    "weight",
	    "strength",
	    "dexterity",
	    "constitution",
	    "intelligence",
	    "wisdom",
	    "charisma",
	];

	// Loop until all fields have been set or Done and print fields which have not been set
	loop {
	    println!("Enter a Character Attribute followed by the associated Value\n");
	    println!("Or 'done' when you are finished\n");
	    println!("Unset Attributes:\n {}", unset_fields.join(","));

	    let mut input = String::new();
	    std::io::stdin().read_line(&mut input).expect("Failed to read input");

	    let input = input.trim();

	    if input == "done" {
		break;
	    }
	    
	    // Split the user's input into 'field' and 'value'
	    let mut parts = input.splitn(2, " ");
	    let field = parts.next().unwrap();
	    let value = parts.next().unwrap();

	    // Use a Match statement to set appropriate value to fields
	    match field {
		"name" => character.name = value.to_string(),
		"class" => character.class = value.to_string(),
		"race" => character.race = value.to_string(),
		"level" => character.level = value.parse().expect("Failed to parse level"),
		"age" => character.age = value.parse().expect("Failed to parse age"),
		"size" => character.size = value.to_string(),
		"height" => character.height = value.to_string(),
		"weight" => character.weight = value.parse().expect("Failed to parse weight"),
		"strength" => character.strength = value.parse().expect("Failed to parse strength"),
		"dexterity" => character.dexterity = value.parse().expect("Failed to parse dexterity"),
		"constitution" => character.constitution = value.parse().expect("Failed to parse constitution"),
		"intelligence" => character.intelligence = value.parse().expect("Failed to parse intelligence"),
		"wisdom" => character.wisdom = value.parse().expect("Failed to parse wisdom"),
		"charisma" => character.charisma = value.parse().expect("Failed to parse charisma"),
		_ => {}
	    }

	}
    }

}


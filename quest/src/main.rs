use std::io;


struct Character {
    name: String,
    class: String,
    level: i32,
}


impl Default for Character {
    fn default() -> Self {
        Character {
		name: String::from(""),
		class: String::from(""),
		level: 0,
        }
    }
}


fn main() {
    // Use Default() to create a new instance
    let mut character = Character::default();

    // Create a vector of field names
    let fields = vec![
	"name",
	"class",
	"level",
    ];

	println!("Remaining fields: {:?}", fields);

    // Use a loop to continuously prompt the user for input
    loop {
	println!("Please enter the name of the field you want to set (or 'done' to exit):");

	// Read the user's input from stdin
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	// Trim trailing white space from input
	let input = input.trim();

	// Check for "done" to break loop
	if input == "done" {
	    break;
	}

	// Otherwise check if the input is is a valid field name
	if !fields.contains(&input) {
	    // If field name is invalid print error message and continue the loop
	    println!("'{}' is not a valid field name.", input);
	    continue;
	}

	// If field name is valid prompt the user for a value
	println!("Please enter the character's {}:", input);

	// Read the user's input value
	let mut value = String::new();
	io::stdin().read_line(&mut value).unwrap();

	// Trim the trailing white space from the value
	let value = value.trim();

	// Set field on the character instance
	match input {
	    "name" => character.name = value.to_owned(),
	    "class" => character.class = value.to_owned(),
	    "level" => character.level = value.parse().unwrap(),
	    _ => (),
	}

	// Create a vector for the remaining fields
	// iterate and filter out any character.[field] that has a value
	let remaining_fields: Vec<&&str> = fields
	    .iter()
	    .filter(|field| { 
		match field {
		    &&"name" => character.name == "",
		    &&"class" => character.class == "",
		    &&"level" => character.level == 0,
		    _ => false,
		    }
		})
	    .collect();


	// Print a message indicating the remaining fields to be filled
	println!("Remaining fields: {:?}", remaining_fields);
    }
}
// test

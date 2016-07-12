fn valid(element:&str, symbol:&str) -> bool {

	let mut found_first_char = false;
	let mut found_second_char = false;
	let first_char = symbol.chars().nth(0).unwrap().to_string().to_lowercase();
	let second_char = symbol.chars().nth(1).unwrap().to_string().to_lowercase();

	for character in element.chars() {
		if found_first_char && found_second_char == false {
			if character.to_string().to_lowercase() == second_char {
				found_second_char = true;
			}
		}

		if found_first_char == false {
			if character.to_string().to_lowercase() == first_char {
				found_first_char = true;
			}
		}
	}

	if found_first_char && found_second_char {
		return true;
	}

	return false;
}

fn main() {
	println!("{}", valid("Spenglerium", "Ee"));
	println!("{}", valid("Zeddemorium", "Zr"));
	println!("{}", valid("Venkmine", "Kn"));
	println!("{}", valid("Stantzon", "Zt"));
	println!("{}", valid("Melintzum", "Nn"));
	println!("{}", valid("Tullium", "Ty"));

}

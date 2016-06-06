
fn print_non_wining_places(won_place: i32) {
	
	let mut display = String::new() ;
	let mut string_number ;
	
	for x in 0..200 {
		if x != won_place && x != 0 {
			string_number = x.to_string() ; 
			let mut copy_string_number = x.to_string() ;
			let last_char = copy_string_number.pop();
			let penultimate_char = copy_string_number.pop();
			
			if let Some(x) = penultimate_char {
				if x == '1' && is_last_digit (last_char) {
						display = append(display, string_number, "th") ;
					}
					else {
						display = append(display, string_number, &get_print_letters(last_char)) ;
					}
			
			} else {
				display = append(display, string_number, &get_print_letters(last_char))
			}
		}
	}
	println!("{}", display ) ;
}

fn append(display:String, string_number:String, append_str:&str) -> String {
	//&string_number converts String to &str type
	display + &string_number + append_str + ", "
}

fn is_last_digit(last_char:Option<char>) -> bool {

	match last_char {
		Some(x) => 	if x == '1' || x == '3' || x == '2' {
						true
					}
					else {
						false
					},
		None => 	{ false } ,
	
	}
}

fn get_print_letters(last_char:Option<char>) -> String {

	let result ;
	match last_char {
	
		Some(x) => result = match x {
			'1' => "st" ,
			'2' => "nd" ,
			'3' => "rd" ,
			_ => "th" ,
		},
		None => { result = "" },
	}
	result.into()
}

fn main() {
	print_non_wining_places(2)
}
fn convert(input:&str){
    let length = input.len() ;
    let number = input[0..length-2].parse::<f32>().unwrap() ;
	let conversion_direction = &input[length-2..length] ;

	let result:Option<f32> = match conversion_direction {
		"rd" => Some(number * ( 180.0 / 3.1416)) ,
		"dr" => Some(number * ( 3.1416 / 180.0 )) ,
		_ => None ,
	} ;

	if result == None {
		println!("No candidate for conversion" );
	}
	else {
		println!("{0}{1}", result.unwrap() , conversion_direction[1..2].to_string() );
	}
}

fn main() {
    convert("3.1416rd") ;
    convert("90dr") ;
    convert("100cr") ;
}

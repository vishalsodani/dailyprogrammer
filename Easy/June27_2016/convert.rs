fn convert(input:String){
    let length = input.len() ;
    let number = &input[0..length-2].parse::<f32>().unwrap() ;

    if input.ends_with("rd") {
        let x =  number * ( 180.0 / 3.1416) ;
        println!("{0}{1}", x, "d" );
    }
    else if input.ends_with("dr") {
        let x = number * ( 3.1416 / 180.0 ) ;
        println!("{0}{1}", x, "r" );
    }
    else {
        println!("No candidate for conversion" );
    }

}

fn main() {
    convert("3.1416rd".to_string()) ;
    convert("90dr".to_string()) ;
    convert("100cr".to_string()) ;
}

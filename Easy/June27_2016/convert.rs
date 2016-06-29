fn convert(mut input:String){
    let to = input.pop().unwrap() ;
    let from = input.pop().unwrap() ;
    let number =input.parse::<f32>().unwrap() ;

    if from == 'r' && to == 'd' {
        let x =  number * ( 180.0 / 3.1416) ;
        println!("{:?}", x );
    }
    else if from == 'd' && to == 'r' {
        let x = number * ( 3.1416 / 180.0 ) ;
        println!("{:?}", x );
    }

}


fn main() {
    convert("3.1416rd".to_string()) ;
    convert("90dr".to_string()) ;
}

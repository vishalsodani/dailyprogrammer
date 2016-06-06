fn test_magic_square()-> bool {
    let magic_square = vec![8, 1, 6, 3, 5, 7, 4, 2, 2] ;
    let check_length = 3 ;
    let mut start = 0;
    let mut sum = 0;

    for number in magic_square.iter() {
            if start >= check_length {
                if sum != 15 {
                    return false;
                }
                start = 0 ;
                sum = 0;
            }
            sum = sum + *number ;
            start = start + 1;
    }
	//columns
	for x in 0..check_length {
		sum = magic_square[x] + magic_square[x+3] + magic_square[x+3+3];
		if sum != 15 {
			return false;
		}
	}
	//diagonals
	if magic_square[0] + magic_square[4] + magic_square[8] != 15 {
		return false;
	}
	if magic_square[2] + magic_square[4] + magic_square[6] != 15 {
		return false;
	}
		true
	}

	fn main(){
		let result = test_magic_square() ;
		println!("{:?}", result );
	}

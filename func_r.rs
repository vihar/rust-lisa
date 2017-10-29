fn main(){
	let a = plus_one(1);
	println!("The value of a is returning from a function : {}",a );
	let b = plus_two(5);
	println!("The value of b is returning from a function : {}",b );
}

fn plus_one(a: i32) -> i32 {
    a + 1 //no ; means an expression, return a+1
}

fn plus_two(a: i32) -> i32 {
    return a + 2; //return a+2 but bad practice, 
    //should use only on conditional returnes, except it's last expression 
}
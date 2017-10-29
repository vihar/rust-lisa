# Introduction/History


Rust is a systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using Garbage Collection.

Rust is a very modern language. It uses LLVM on its backend. Rust supports a mixture of imperative procedural, concurrent actor, object-oriented and pure functional styles. It also supports generic programming and metaprogramming, in both static and dynamic styles.

### Installation

If you're on Linux or a Mac, all you need to do to install Rust is power up your terminal and run :

```
$ curl -sf -L https://static.rust-lang.org/rustup.sh | sh
```

If you're on Windows, please download either the 32-bit installer or the 64-bit installer and run it.

If you've got Rust installed, you can open up a shell, and type this:

$ rustc --version
rustc 1.20.0

Which gives you the rust version installed.

Also verify installion guide << Link >>

## Getting Started

### Say hello to Rust !

Now let's get started with our traditional program, "Hello World "
Our program begins with the main function, statements after the main function will be executed while complied.

println! is called as a macro which is similar to a function but ends with an exclamation/bang "!", but instead of a function call, macros are expanded into source code that gets compiled
with the rest of program.

Now to run our program first, we need to compile our code using the command 'rustc file_name.rs'
which then generates a binary file which should to executed.

**helloworld.rs**

```rust
fn main(){
	println!("Hello World!");
}
```
```
$ ./helloworld

Hello World!
```


### Comments

As we know every programming language needs comments, Rust supports a few different varieties of comments, which are ignored by the compiler.
These are the types of comments which we often use.

```
// — Single line comments which go the end of the line.
/* */ — Multiple line comments which go to the closing delimiter.
```
**comments.rs**

```rust
//Line comment which go to the end of the line.
fn main(){
	println!("Hello World, I am not given a comment.");
	//println("I am ignored by the compiler");
	/*
	Large chunck of code can be commented this way !
	So it takes only a few symbols to comment out all these lines.
	*/
}
```

### Variables

A variable is a piece of storage that contains a value. In Rust, it’s easy to define a variable and set a value to it. Imagine you want to store the number 1in a variable called one. 

```rust
let one  = 1;
```

As simple as that! You just assigned the variable one to the value 1, let is used to introduce a binding.

### Type Annotation

Rust offers you to declare variables of your own size, as rust is a statically typed language we can specify our types up front, and they’re checked at compile time.

Here is an example of declaring 32-bit signed integer. We use let for binding, followed by a variable name and the type, size which come after a colon (:).

```rust
let x: i32 = 19;
```

Rust has many different primitive integer types. They begin with i for signed integers and u for unsigned integers. The possible integer sizes are 8, 16, 32, and 64 bits.

### Mutability

In Rust when you declare a variable, the bindings are immutable which means we cant change the value of the variable.

```rust
fn main(){
	let x = 5;
	x = 10;
}
```
This piece of code gives an error it shows re-assignment of immutable variable `x` !

```
 --> variables.rs:3:2
  |
2 | 	let x = 5;
  | 	    - first assignment to `x`
3 | 	x = 10;
  | 	^^^^^^ re-assignment of immutable variable

```

If you want a binding to be mutable, you can use keyword mut.

```rust
fn main(){
let mut x = 5; // mut x: i32
x = 10;
}
```

### Data Types

Primitive Data Types in Rust

1. Boolean

A standard boolean. Can be either true or false.

```rust
let t = true;
let f = false;
```

2. char

A 4 byte character.

```rust
let a = 'a';
let b = 'b';
let love = '<3';
```

3. Numeric Types

Intergers

These types include i8, i16, i32, i64, isize, u8, u16, u32, u64, usize. The letter denotes whether it is signed (i) or unsigned (u), and the number denotes the size of the integer. So the type i8 is an 8 bit, integer and a u64 is an unsigned, 64 bit integer. isize and usize are dependent upon the architecture of the computer.

```rust
let date = 19;
let life = 35;
let miles = 91010101;
```

Floats

These types include f32 and f64. A floating point number is what we typically refer to as a decimal.

```rust
let pi = 3.14;
let e = 2.718;
```

4. Array

An array is fixed-size, collection of same-type elements.

It is declared like:

```rust
let name: [type; size] = [elem1, elem2, elem3, elem4];

let my_array: [i32; 3] = [19, 24, 2];
println!("The first element of the array is: {}", array[0]);
```

5. Tuple

Tuples : fixed-size ordered list of elements of different(or same) data types

Declaration of tuples is similar to an array. Instead we use "( )" instead of "[ ]".

```rust
let a = (1, 1.5, true, 'a', "Hello, world!");
// a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello, world!"
```

Tuples are also immutable by default and even with mut, its element count can not be changed. Also if you want to change an element’s value, new value should have the same data type of previous value.

6. Strings 

An str is a "String Slice," and is the most primitive string type.

```rust
let str = "Hello! I'm a String";
```

Now something that trips some people when getting started with rust is printing variables. In order to print this we would need to write:

```rust
println!("The value of our variable is: {}", str);
```

Which is similar to Python formatting.

7. Vectors

In Rust vectors store their contents as contiguous arrays of T on the heap. This means that they must be able to know the size of T at compile time (that is, how many bytes are needed to store a T?).

You can create them with the vec! macro:
```rust 
let v = vec![1, 2, 3, 4, 5];

println!("The third element of v is {}", v[2]);
```


### Control Flow

If-Else Statements

We always need the ability to check the conditions and change the behavior of the program accordingly. The conditional statements give us the ability; the simplest form is the ‘if’ statement.

The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

Example 

```rust
let team_size = 7;
if team_size < 5 {
    println!("Small");
} else if team_size < 10 {
    println!("Medium");
} else {
    println!("Large");
}
```

Multiple conditions with elif

We can have multiple conditions by combining if and else in an else if expression. For example:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Loops

Rust currently provides three approaches to performing some kind of iterative activity. They are: loop, while and for. Each approach has its own set of uses.

1. While

Rust also has a while loop. It looks like this:


```rust
fn main() {
	let mut a = 1;
	while a <= 10 {
		println!("Current value : {}", a);
	a += 1; //no ++ or -- in Rust
	}
}
```

2. For

For loops in rust is used for iterating based on the given start postition and end position. Rust’s for loop doesn’t look like this “C-style” for loop, it looks like this.

```rust
fn main(){
	for x in 0..10 {
    println!("{}", x); // x: i32
	}
}
```


### Functions

Every Rust program has at least one function, the main function:

```rust
fn main() {
}
```
In rust to declare functions we use fn keyword followed by the function name, some parentheses where we include the arguments if we have any.

Now let's write a function where we can add two integers.

```rust
fn main() {
    print_sum(5, 6);
    print_sum(19, 24);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}
```
You separate arguments with a comma, both when you call the function, as well as when you declare it.

Functions with return type.

By default functions return empty tuple (). If you want to return a value, return type must be specified after -> .

```rust
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
```



















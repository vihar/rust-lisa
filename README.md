# Introduction/History


Rust is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

Let’s get started with some basics!

### Say hello to Rust !

Now lets get started with our traditional program, "Hello World "
Our program starts with main function, statements after the main function will be exectued while complied.

println! is called as macro which is similar to a function but ends with an exclamation/bang "!",
but instead of function call, macros are expanded into source code that gets compiled
with the rest of program.

Now to run our program first we need to compile our code using the command 'rustc file_name.rs'
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

As we know every programming language needs comments, Rust supports a few different varities of comments, which are ignored by the compiler.

These are the types of comments which we often use.

```
// - Single line comments which go the end of the line.

/**/ - Multiple line comments which go to the cloing delimeter.
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

### Varaibles

A variable is a piece of storage that contains a value. In Rust it’s easy to define a variable and set a value to it. Imagine you want to store the number 3 in a variable called three. 

```rust
let one  = 1;
```

As simple as that ! You just assigned the variable three to the value 3, let is used to introduce a binding.

### Type Annotation

Rust offers you to declare variables of your own size, as rust is a statically typed language we can
specify our types up front, and they’re checked at compile time.

Here is an example of declaring 32-bit signed integer. We use let for binding, followed by variable name and the type, size which come after a colon (:).

```rust
let x: i32 = 19;
```

Rust has many different primitive integer types. They begin with i for signed integers and u for unsigned integers. The possible integer sizes are 8, 16, 32, and 64 bits.

### Mutability

In Rust when you declare a variable, the bindings are immutable which means we cant change the value 
of the variable.

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
let mut x = 5; // mut x: i32
x = 10;
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

5. Strings 

A str is a "String Slice", and is the most primitive string type.

```rust
let str = "Hello! I'm a String";
```

Now something that trips some people when getting started with rust is printing variables. In order to print this we would need to write:

```rust
println!("The value of our variable is: {}", str);
```

Which is similar to Python Formmating.

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





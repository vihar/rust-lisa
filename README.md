# Introduction to Rust

Rust is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

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







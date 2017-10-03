# Introduction to Rust

Rust is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

### helloworld.rs


```rust
fn main(){
	println!("Hello World!");
}
```

Now lets get started with our traditional program, "Hello World "
Our program starts with main function, statements after the main function will be exectued while complied.

println! is called as macro which is similar to a function but ends with an exclamation/bang "!",
but instead of function call, macros are expanded into source code that gets compiled
with the rest of program.

Now to run our program first we need to compile our code using the command 'rustc file_name.rs'
which then generates a binary file which should to executed.


./helloworld
Hello World!


**9th Feb 2025**

Link: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

```
use std::io
```
The io library comes from the standard library, known as std.

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the  [standard library documentation](https://doc.rust-lang.org/std/prelude/index.html).

```
println!()
```
is a macro that prints strings

***What is a macro?*** 

A macro in Rust is a piece of code that generates other code at compile time, essentially allowing you to write reusable code patterns that can be expanded into more complex code structures, enabling a form of metaprogramming where you can manipulate syntax directly; it's a powerful tool to reduce code repetition and create more concise syntax, often recognized by the "!" exclamation mark at the end of its name, like println!.


**Storing Values with Variables**

Next, we’ll create a variable to store the user input, like this:
```
    let mut guess = String::new();
```
Now the program is getting interesting! There’s a lot going on in this little line. We use the let statement to create the variable. Here’s another example:

```
let apples = 5;
```

This line creates a new variable named apples and binds it to the value 5. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change. We’ll be discussing this concept in detail in the “Variables and Mutability” section in Chapter 3. 

To make a variable mutable, we add mut before the variable name:

```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
Note: The // syntax starts a comment that continues until the end of the line. Rust ignores everything in comments. We’ll discuss comments in more detail in Chapter 3.

Returning to the guessing game program, you now know that let mut guess will introduce a mutable variable named guess. The equal sign (=) tells Rust we want to bind something to the variable now. On the right of the equal sign is the value that guess is bound to, which is the result of calling String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.

In full, the ```let mut guess = String::new();``` line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!

Receiving User Input
Recall that we included the input/output functionality from the standard library with use std::io; on the first line of the program. Now we’ll call the stdin function from the io module, which will allow us to handle user input:
```
    io::stdin()
        .read_line(&mut guess)
```
If we hadn’t imported the io library with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin. The stdin function returns an instance of ```std::io::Stdin```, which is a type that represents a handle to the standard input for your terminal.

Next, the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that, like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. (Chapter 4 will explain references more thoroughly.)


Handling Potential Failure with Result
We’re still working on this line of code. We’re now discussing a third line of text, but note that it’s still part of a single logical line of code. The next part is this method:
```
        .expect("Failed to read line");

```


```loop {} ``` is a way to loop a piece of code until the program forces to break out of the loop

```match condition {}``` is a way to handle conditional statement.




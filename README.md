# VigScript

Vigscript is an interpreted language coded in Rust, to explore the Rust language.

## Using Vigscript

*Disclaimer: There are no releases at the moment.*
You have to build it yourself.
1. Clone the repo: `git clone git@github.com:Vigintillionn/vigscript.git`
2. Make sure you have [rust](https://rustup.rs/) installed
3. Cd into the folder `cd vigscript`
4. Build the cargo with `cargo build`
5. The executable is found in `target/debug/`
6. You can move the executeable to your path if you wish

### Interpreting a file

A file can simply be interpreted by running the executeable with an argument, the file's path. `./vigscript main.vig`
The parser can only parse files with the extension `.vig`

### Using the repl

You could also use the repl, using `./vigscript` with no argument. Right now it evaluates line per line, not allowing you to indent statements like and `if` or `func` statement. *This will be fixed in the future*

## Why use vigscript?

I don't know. It's a terrible language, which is totally not optimised but have fun.

## Syntax

An example can be found in the `example` directory.
I wanted to combine the syntax of some of my favorite languages. You will probably recognize a lot.
*Disclaimer: The syntax is subject to change in the future.*

### Variables

Vigscript, just like rust, makes variables immutable by default. There are 2 ways of making a variable:
```rs
let x = 5;
let y;
const PI = 3.14;
```
The let keyword is allowed to be null, but will afterwards only be able to be assigned once. As the variable will be immutable.
A constant variable must always have a variable and should be written in UPPERCASE.
If you wish to create a mutable variable, the `mut` keyword must be used.

Variable declarations must always end with a `;`. An assignment may not end with one, as it is handled as an expression.

```rs
let x = 5;
x = 10 // Not OK

let mut x = 5;
x = 10 // OK

let mut x;
let mut y;
x = y = 3
```

### Data Structures
#### Strings
Strings can be created with
```rs
let x = "hello";
```

You may join two strings with the `+` operator
```rs
let x = "hello";
let y = "world";
let z = x + " " + y; // hello world
```
### Numbers
All numbers are floating points numbers by default. *This is subject to change in the future*
```rs
let x = 5;
```
Standard mathematical operations can be done using numbers. This includes `+`, `-`, `/`, `*` and `%`.

### Booleans
Booleans are just your standard boolean values. All values not equal to `false` or `null` are treated as `true` in conditions.
```rs
let x = true;
let y = false;
```

### Objects
Objects are just like other languages. Currently the member acces expression is not functioning like intended but is a priority to be fixed.
```rs
let obj = {
  bar: {
    foo: 5
  },
  fizz: 10
};

print!(obj::bar)
```
Members are accessed with the member access operator `::`.

## Control Flow
### If statements
If statemens are very intuitive:
```rs
let foo = 5;

if foo == 5 {
  if foo > 3 {
    print!("success")
  } else {
    print!("failure")
  }
}
```

### For loops
At the moment you may only loop over an array like so:
```rs
let arr = [1, 3, 5, 7];
for let i in arr {
  print!(i)
}
```
The variable `i`, in this case, will take on the value of the next index for each iteration. The above snipper will print in the console `1, 3, 5, 7`.

### Functions
Functions are also very intuitive
```rs
func add(a, b) {
  print!(a + b)
}

add(2, 3)
```
A function may have any number - or zero - parameters. The function can be called like most languages. If you wish to return a value in a function you can do with the `ret` keyword. The `ret` keyword is treated as a statement and must thus be ended with a `;`.
```rs
func add(a, b) {
  ret a + b;
}
print!(add(2, 3)) // Prints 5
``` 
If the last statement of the function is an expression the result of that expression will be returned, if the function hasn't returned earlier.
```rs
func add(a, b) {
  a + b
}
print!(add(2, 3)) // Prints 5
```

## Native Functions
Vigscript comes with a number of native functions which will later be extended upon. *Disclaimer: If I get to it, these native functions will be moved to the standard library*
Native functions can be recognized by the trailing `!`.

### Print!
The `print!` function will simply print it's arguments to the console.
```rs
print!(3, 5) // 3 5
```

## Helpers
Vigscript also comes with a number of helpers. Some of these will however later on be moved to native methods, once they are implemented.
### Array
The array helper is used to do various operations with an array.
#### Array::new(n)
Using the new method you can create a new array with a set amount of elements.
```rs
let arr = Array::new(5);
print!(arr) // [Null, Null, Null, Null, Null]
```
#### Array::from(...args)
Using the from method you can pass in a set amount of arguments which will be added as elements to the array.
```rs
let arr = Array::from(1, 3, 5, 7);
print!(arr) // [1, 3, 5, 7]
```

#### Array::has(Array, el)
The has method will return a boolean if the array contains said element.
```rs
let arr = [1, 3, 5, 7];
print!(Array::has(arr, 5)) // true
```
**Note** This method has almost identical behaviour as the `in` keyword but the `in` keyword will return the value if it was found and `null` if it hasn't been found. Example:
```rs
let arr = [1, 3, 5, 7];
print!(5 in arr) // 5
print!(2 in arr) // null
print!(Array::has(arr, 5)) // true
print!(Array::has(arr, 2)) // false
```

#### Array::concat(Array1, Array2)
The concat method will join two arrays into one array.
```rs
let arr1 = [1, 3, 5, 6];
let arr2 = [6, 7, 8];
let res = Array::concat(arr1, arr2);
print!(res) // [1, 3, 5, 6, 6, 7, 8]
```

### Date
The Date helper consists of various (one) methods that interact with the current data
#### Date::now()
Returns the current epoch timestamp in milliseconds
```rs
print!(Date::now()) // 1700518585000
``` 


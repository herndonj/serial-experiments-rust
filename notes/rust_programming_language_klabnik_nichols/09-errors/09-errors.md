---
title: "Ch 09 - Error Handling"
format: 
  html:
    theme: cosmo
    toc: true
    toc-depth: 4
    toc-location: body
editor:
    render-on-save: true
---

## Intro
Rust has two major categories of errors:

1. Unrecoverable: `panic!` (execution stops)
2. Recoverable: `Result<T, E>`

Note other languages don't make this distinction. Note Python's use of exceptions.

## 1. Unrecoverable Errors with `panic!`
Panics happen when:

1. explicitly called - `panic!("I'm crashing!")`
2. when something goes wrong with the program (accessing an array past the end).

- By default, a panic causes Rust to start moving up the stack and cleaning up the data in all the functions. You can stop this by adding `panic = 'abort' to the appropriate `[profile]` in the *Cargo.toml*

```{rust}
//10-call_panic_explicitly.rs

fn main() {
    panic!("crash and burn"); //the backtrace will point here
}
```
- Sometimes when you have a panic, the error message provides a filename and line number for someone else's code- it happens...

- p164 reading a backtrace when you set the RUST_BACKTRACE=1 environment variable. (`RUST_BACKTRACE=1 cargo run`)

- The authors (K&N) note that choosing between a panic and a `Result` depends on the situation. (The following text comes from copilot- but it seems appropriate...) If the code is in a library, it's better to return a `Result` so the calling code can decide what to do. If the code is in a binary, it's better to panic because the binary can't do much with an error other than crash.

## 2. Recoverable Errors with `Result <T, E>`

Sometimes, you want the opportunity to recover from an error with grace. For example, if a opening a file fails, you might want to create the file instead of panicking. Another option is to provide feedback to the user on what's happening (see the guessing game in chapter 2 where the user is told if the program fails to read the line). 

[**The `Result` enum**]{.underline}

The `Result` enum is defined as follows:

```{rust}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The T and E are generic type parameters. T is the type of the value that will be returned in a success case within the Ok variant, and E is the type of the error that will be returned in a failure case within the `Err` variant. In the guessing game (chapter 2, line 21), we called the `.expect("<message>")` method on the Result enum which will return the value in the `Ok` variant if the Result is an `Ok`, or it will panic with the message provided if the Result is an `Err`.

Opening a file is common situation where you might want to use a `Result`. The `File::open` method returns a `Result` enum because it might fail. If the file doesn't exist, the `File::open` method will return an `Err` variant that contains an error message. If the file does exist, the `File::open` method will return an `Ok` variant that contains a `File` instance that we can read from. 

```{rust}
//20-file_open_result.rs 
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

If the code above works, the `T` in the `Result<T, E>` enum will be a file handle because that's what the `use std::fs::File` module returns. If the code above fails, the `E` in the `Result<T, E>` enum will be `std::io::Error`. The code above doesn't do anything with the `Result` enum, so we look to next code block to take different actions based on the variant of the `Result` enum.

```{rust}
//30-file_open_with_match_expression_error_handling.rs
// aka Listing 9-4 on page 166

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

The code above uses a `match` expression to handle the `Result` enum. If the `Result` enum is an `Ok` variant, the `match` expression will return the file handle contained in the `Ok` variant. If the `Result` enum is an `Err` variant, the `match` expression will panic with the error message contained in the `Err` variant.

### Matching on Different Errors

Often you want different behavior for different errors. For example, if the file doesn't exist, you might want to create it or if you can't create it, you might want to alert the user. The code below shows how to handle different errors differently using an inner `match` expression.

p. 167 K&N

```{rust}
//40-file_open_with_match_expression_error_handling.rs

use std::fs::File;
use std::io::ErrorKind; // we need to know what type of io Error

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

The error variant of `File::open` returns a struct value `io::Error` which has a method called `kind` that we call in the outer match to get an `io::ErrorKind` value. The `io::ErrorKind` enum (in the standard library) has many variants that spell out what can go wrong in an io operation. We use the `ErrorKind::NotFound` here. 

NOTE- this is a lot to parse if you are new to Rust. It's one of the many times where a familiarity with the standard library and the standard return values grows more familar with usage over time. Don't sweat the details early on...

Continuing the dialog above- if we know that the operating system couldn't find the file and returned the `ErrrorKind::NotFound` variant, we try to create the file with the `File::create` method. If the file creation succeeds, we return the file handle. If the file creation fails, we panic with the error message. If the error is not the `ErrorKind::NotFound` variant, we panic with the error message.



p. 168 K&N

#### Alternatives to Using `match` with `Result<T, E>`

The `match` code is a bit verbose (but it's also very clear). Chapter 13 (<sigh> - another reference to future chapters) discusses using closures to handle errors. The code below shows how to use closures in a way that concisely handles errors without a lot of nested `match` expressions.

NOTE- 2024-01-20 - Just wondering if you'll come back to these notes when you reach Chapter 13.

The code below uses `closures` and the `unwrap_or_else` method to handle errors. The `unwrap_or_else` method takes a closure as an argument and calls the closure if the `Result` is an `Err` variant. If the `Result` is an `Ok` variant, the `unwrap_or_else` method returns the value contained in the `Ok` variant. 

```{rust}
//50-file_open_with_closure_error_handling.rs

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
Besides being concise- the code helps clean up complex nested match expressions.

#### Shortcuts for Panic on Error: `unwrap` and `expect`

Once more, `match` can be verbose. One quick subsitute is to use one of `Result<T,E>`'s methods: `unwrap` or `expect`. Both methods are defined on the `Result<T,E>` enum. If the `Result` is an `Ok` variant, both methods return the value contained in the `Ok` variant. If the `Result` is an `Err` variant, both methods panic with the error message contained in the `Err` variant.

```{rust}

//60-file_open_with_unwrap_error_handling.rs

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```
The code above will open the file or panic with the error message contained in the `Err` variant. 

p. 169 K&N

PERSONALLY - the `expect` method on the `Result<T,E>` enum is a bit more user focused in that it allows you to specify the error message for the end user.

The `expect` method is similar to the `unwrap` method, but it allows you to specify the error message. The code below shows how to use the `expect` method.

```{rust}
//70-file_open_with_expect_error_handling.rs

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

The book notes that most programmers use `expect()` over `unwrap()` because it allows you to specify what the end user should expect to see when the program fails thus helping with the later debugging.

### Propagating Errors p. 169

When you're writing a function that calls another function that might fail, instead of handling the error within the function, you can return the error to the calling function so it can decide what to do. This is known as propagating the error. This is helpful when you're writing a library that will be used by other programs. The calling program might have more context to decide what to do with the error (you don't want to have to worry about what's happening inside the library in great detail- you just want to know why it failed and whether the caller can do anything helpful in response).

The code below shows how to propagate an error from a function that reads a username from a file to the calling function.

```{rust}
//80-read_username_from_file_propagating_error.rs
// aka Listing 9-6 on pages 169-170 K&N

use std::fs::File;
use std::io::{self, Read}; // I need more explanation here...

fn read_username_from_file() -> Result<String, io::Error> {
    // note that the return type is either a String (the username) or
    // we use io::Error because both File::open and read_to_string return
    // io::Error if they fail.
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), //the return here passes back the error
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), //we don't specify the return because the last line returns by default
    }
}
```
The power of the code above is that the calling function can decide what to do with the error. If the file won't open or the file doesn't have a username (both possible), it can choose to panic!, lookup the username elsewhere, or some other outcome. (170)

**TAKEAWAY** - This pattern is of propagating errors is so common in Rust that Rust provides the `?` operator to make this easier.

#### A Shortcut for Propagating Errors: the `?` Operator (171 K&N)

The code below uses the same `read_username_from_file` function as above, but it uses the `?` operator to propagate the error instead of using `match` expressions. The `?` operator replaces the match statements in the previous example. If the value is `Ok` the value inside the `Ok` will get returned. If the value is `Err`, the `Err` value goes through the `from` function (defined in the `From` trait in the standard library... which the K&B haven't referenced previously?...) which converts the error received into the error specified in the return type of the function. This is handy because it means you don't have to specify the return type of the function in the `Err` case (and can reduce some of the complexity of having to deal with a range of errors.)

J's note- this reminds me of Python's `try` and `except` statements. 


```{rust}
//90-read_username_from_file_propagating_error_with_question_mark.rs
// aka Listing 9-7 on page 171

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    // the ? operator is called the "try" operator. 
    // If the Result is an Err variant, the ? operator will 
    // the ? operator will return the Err value from the current function for the caller
    // to handle. If the Result is an Ok variant, the ? operator will return the value
    // inside the Ok variant to the calling function.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

p.172 demonstates how to chain method calls (example 100) to make this shorter before finally revealing that reading a file into a string is so common that the standard library provides a `fs::read_to_string` function that does the same thing as the `read_username_from_file` function above. The code below shows how to use the `fs::read_to_string` function. (note - you can't see all the error handline in example 110)

**Chaining method calls to shorten code**
```{rust}
//100-read_username_from_file_chaining_methods_and_propagating_error_with_question_mark.rs
// aka Listing 9-8 on page 172

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}
```


**SHORTCUT: Using the `fs::read_to_string` function as a shortcut**

Most people will never use all the code above with the explicit error handling, but will use the `fs::read_to_string` function instead. The code below shows how to use the `fs::read_to_string` function.
```{rust} 
//110-using_fs_read_to_string_as_a_shortcut.rs
// aka Listing 9-9 on page 172

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```


#### Where the ? Operator Can Be Used

Note- you can only use the `?` operator in functions that have a return type of `Result<T, E>` or `Option<T>`. You can't use the `?` operator in `main` usually because `main` has a default return type of `()`. 

```{rust}
error[E0277]: the `?` operator can only be used in a function that returns `Result` (or `Option`)
 --> src/main.rs:2:5

use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

The solution is to use ? in a function that returns `Result` or `Option` and then call that function from `main` as we've done in the previous examples. The other is to stick with `match` expressions or use the `Result<T, E>` methods to handle things.

p. 173

K&B go on to discuss possibilities for using the `?` operator before turning to more philosophical discussions about error handling.
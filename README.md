
# Overview

## Why this repo?

Sometimes you want to translate python directly into rust (usually for increased speed).  As an example, the code in `Cargo.toml` and `src/lib.rs` is the rust implementation of the Python code below (via the rust `pyO3` crate)...

## Assumptions

All examples assume you're running linux (although Windows can also be used with small modifications).

## Python code

`src/lib.rs` translates the following python into rust...

```python
class Person:
    def __init__(self, first_name, last_name, age):
        self.first_name = first_name
        self.last_name = last_name
        self.age = int(age)

    def have_birthday(self):
        self.age += 1

    def __str__(self):
        return f"<Person {self.first_name} {self.last_name} is {self.age} years old>"

    def __repr__(self):
        return self.__str__()
```

## A trivial rust implementation

When you run `cargo build --release`, it will build `src/lib.rs` into a linux library.  After you copy the `target/release/libmyrust_person.so` library into your working directory, it can then be imported and used just like the python code above with:

```python
>>> from libmyrust_person import Person
>>>
>>> person = Person("John", "Smith", 25)
>>> person.have_birthday()
>>>
>>> person
'<Person John Smith is 26 years old>'
>>>
```

# pyO3

## Maturin

Even though you can use `Maturin` to automate some things, it's technically not required if you follow this example completely.

## Limitations

When you call the pyO3 `target/release/libmyrust_person.so` library from python, remember that any relevant python variables must be translated into rust via pyO3.  Since we haven't implemented much logic in rust, merely compiling this translated python `class Person()` code is not that much faster.

To get increased speed from a rust pyO3 implementation, you need to write major functionality directly in rust, and then leverage those reimplemented rust functions to the point that you get more speed than the overhead incurred by merely mapping python `Person()` instance variables into rust.


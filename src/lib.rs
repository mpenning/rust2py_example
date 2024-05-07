#![allow(clippy::pedantic)]
use pyo3::prelude::*;

// The exported python class will be named "Person()"
#[pyclass(name = "Person")]
struct PersonPy {
    #[pyo3(get, set)]
    first_name: String,
    last_name: String,
    age: u8
}

#[pymethods]
impl PersonPy {
    #[new]
    fn init(first_name: String, last_name: String, age: u8) -> Self {
        // return a PersonPy struct...
        PersonPy { first_name, last_name, age }
    }

    fn have_birthday(&mut self) {
        // this needs (&mut self) because we modify self.age in the function...
        self.age += 1;
    }

    // emulate a python __repr__() method
    fn __repr__(&self) -> String {
        format!("<{} {}, is {} years old>", self.first_name, self.last_name, self.age)
    }

    // emulate a python __str_() method
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[pymodule]
// Note that we call this library 'myrust_person' in Cargo.toml...
#[pyo3(name = "libmyrust_person")]
fn myrust_person(_py: Python, m: &PyModule) -> PyResult<()> {
    // expose the PersonPy struct
    m.add_class::<PersonPy>()?;
    Ok(())
}

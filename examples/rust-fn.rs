use inline_python::{python, Context};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn rust_print(x: i32) {
	println!("rust: x = {}", x);
}

fn main() {
	let c = Context::new();

	c.add_function(wrap_pyfunction!(rust_print, Python::acquire_gil().python()).unwrap());

	c.run(python! {
		x = 123
		print("python: x =", x)
		rust_print(x)
	});
}

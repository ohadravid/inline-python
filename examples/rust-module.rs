use inline_python::{python, Context};
use pyo3::{prelude::*};

#[pyfunction]
fn rust_print(x: i32) {
	println!("rust: x = {}", x);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_printer(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(rust_print, m)?)?;

    Ok(())
}

fn main() {
	let c = Context::new();

	c.add_module(pyo3::wrap_pymodule!(rust_printer));

	c.run(python! {
		x = 123
		print("python: x =", x)
		rust_printer.rust_print(x)
	});
}

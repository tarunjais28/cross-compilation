use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::fs;

pub fn process_pyth_script() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Read Python code from a file
        let python_code = fs::read_to_string("script.py").expect("Unable to read Python script");

        // Compile and load the Python code as a module
        let module = PyModule::from_code_bound(py, &python_code, "script.py", "script_module")?;

        // Call the Python function `greet`
        let greet = module.getattr("greet")?;
        let result: String = greet.call1(("Rust",))?.extract()?;

        // Print the result from the Python function
        println!("{}", result); // Output: Hello, Rust!

        Ok(())
    })
}

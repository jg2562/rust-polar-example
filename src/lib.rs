mod to_py;
mod to_rust;
mod error;

use polars::{prelude::*, df};
use pyo3::prelude::*;
use crate::to_py::df_to_py;
use crate::to_rust::to_rust_df;


#[pyfunction]
fn create_df() -> PyResult<Vec<PyObject>> {
	let df: DataFrame = df!(
		"A" => &[1,2,3],
		"B" => &[4,5,6]
	).unwrap();
	df_to_py(df)
}

fn calculation(series: &Series) -> Series {
	series + 1
}

#[pyfunction]
fn super_advanced_calculation(py_batches: Vec<&PyAny>) -> PyResult<Vec<PyObject>> {
	let mut df = to_rust_df(&py_batches)?;
	df.apply("B", calculation).unwrap();
	df_to_py(df)
}



#[pymodule]
fn tapki_zero(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(create_df, m)?)?;
	m.add_function(wrap_pyfunction!(super_advanced_calculation, m)?)?;
	Ok(())
}

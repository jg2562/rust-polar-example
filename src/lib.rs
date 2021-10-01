mod to_py;

// use polars::{prelude::*, df};
// use pyo3::prelude::*;
// use crate::to_py::to_py_rb;

// #[pyfunction]
// fn create_df() -> PyResult<PyObject> {
// 	let df = df!(
// 		"A" => &[1,2,3],
// 		"B" => &[4,5,6]
// 	).into();
// 	to_py_rb(df)
// }

// #[pymodule]
// fn tapki_zero(_py: Python, m: &PyModule) -> PyResult<()> {
// 	m.add_function(wrap_pyfunction!(create_df, m)?)?;
// 	Ok(())
// }

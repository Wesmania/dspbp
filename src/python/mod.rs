use pyo3::{prelude::*, types::PyBytes, exceptions::PyValueError};

use crate::{blueprint::Blueprint, data::enums::{DSPItem, DSPRecipe}, edit::EditBlueprint};
use std::collections::HashMap;

#[pyclass]
pub struct PyBlueprint(EditBlueprint);

#[pymethods]
impl PyBlueprint {
    #[getter]
    pub fn get_icon_text(&self) -> PyResult<String> {
        self.0.get_icon_text().map_err(ve)
    }

    #[setter]
    pub fn set_icon_text(&mut self, test: &str) -> PyResult<()> {
        self.0.set_icon_text(&test);
        Ok(())
    }

    #[getter]
    pub fn get_description(&self) -> PyResult<String> {
        self.0.get_description().map_err(ve)
    }

    pub fn info(&mut self) -> PyResult<String> {
        self.0.info().map_err(ve)
    }

    pub fn replace_item(&mut self, map: HashMap<DSPItem, DSPItem>) -> PyResult<()> {
        self.0.replace_item(map);
        Ok(())
    }

    pub fn replace_recipe(&mut self, map: HashMap<DSPRecipe, DSPRecipe>) -> PyResult<()> {
        self.0.replace_recipe(map);
        Ok(())
    }
    
    pub fn replace_both(&mut self, r: HashMap<DSPItem, DSPItem>) -> PyResult<()> {
        self.0.replace_both(r);
        Ok(())
    }

    pub fn replace_building(&mut self, map: HashMap<DSPItem, DSPItem>) -> PyResult<()> {
        self.0.replace_building(map).map_err(ve)
    }
}

fn ve(e: anyhow::Error) -> PyErr {
    PyValueError::new_err(e.to_string())
}

#[pyfunction]
fn load(buf: &PyAny) -> PyResult<PyBlueprint> {
    let bytes = buf.downcast::<PyBytes>()?;
    let data = std::str::from_utf8(bytes.as_bytes())?;
    let bp = Blueprint::new(&data).map_err(ve)?;
    Ok(PyBlueprint(EditBlueprint::new(bp)))
}

#[pyfunction]
fn save<'a>(py: Python<'a>, bp: &PyBlueprint) -> PyResult<&'a PyBytes> {
    let data = bp.0.0.into_bp_string().map_err(ve)?;
    Ok(PyBytes::new(py, data.as_bytes()))
}

#[pymodule]
fn dspbp(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(save, m)?)?;
    m.add_class::<PyBlueprint>()?;
    m.add_class::<DSPItem>()?;
    m.add_class::<DSPRecipe>()?;
    Ok(())
}

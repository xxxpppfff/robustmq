use std::{
    fs,
    path,
};

use crate::errors::RobustMQError;

pub fn read_file(path: &String) -> Result<String, RobustMQError> {
    if !path::Path::new(path).exists() {
        return Err(RobustMQError::CommmonError(format!(
            "File {} does not exist",
            path
        )));
    }

    return Ok(fs::read_to_string(&path)?);
}

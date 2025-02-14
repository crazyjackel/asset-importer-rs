use std::iter;

use crate::accessor::Accessors;
use crate::error::Error;
use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Document(gltf_v1_json::Root);

impl Document {
    pub fn from_json(json: json::Root) -> Result<Self> {
        let document = Self::from_json_without_validation(json);
        document.validate()?;
        Ok(document)
    }

    pub fn from_json_without_validation(json: json::Root) -> Self {
        Document(json)
    }

    pub fn into_json(self) -> json::Root {
        self.0
    }

    pub fn as_json(&self) -> &json::Root {
        &self.0
    }

    /// Perform validation checks on loaded glTF.
    pub(crate) fn validate(&self) -> Result<()> {
        use json::validation::Validate;
        let mut errors = Vec::new();
        self.0
            .validate(&self.0, json::Path::new, &mut |path, error| {
                errors.push((path(), error))
            });
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }

    pub fn accessors(&self) -> Accessors {
        Accessors {
            iter: self.0.accessors.iter(),
            document: self,
        }
    }
}

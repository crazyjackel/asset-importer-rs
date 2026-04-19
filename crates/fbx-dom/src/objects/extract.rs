use std::collections::HashMap;

use fbxscii::ElementAttribute;

use super::FbxTryFromReason;

pub trait AttrExtractor {
    fn extract(&self, name: &str) -> Option<&ElementAttribute>;
    fn extract_case_insensitive(&self, name: &str) -> Option<&ElementAttribute>;
}

impl AttrExtractor for HashMap<String, ElementAttribute> {
    fn extract(&self, name: &str) -> Option<&ElementAttribute> {
        self.get(name)
    }
    fn extract_case_insensitive(&self, name: &str) -> Option<&ElementAttribute> {
        self.get(name).or_else(|| {
            self.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(name))
                .map(|(_, v)| v)
        })
    }
}

pub trait AttrExtractorExt {
    fn require_token<'a>(&'a self, name: &'a str) -> Result<&'a str, FbxTryFromReason>;
    fn require_token_case_insensitive(&self, name: &str) -> Result<&str, FbxTryFromReason>;
    fn optional_token<'a>(&'a self, name: &'a str) -> Result<Option<&'a str>, FbxTryFromReason>;
    fn optional_token_case_insensitive<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a str>, FbxTryFromReason>;
    fn optional_tokens<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a [String]>, FbxTryFromReason>;
    fn optional_tokens_case_insensitive<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a [String]>, FbxTryFromReason>;
}

impl<T: AttrExtractor> AttrExtractorExt for T {
    fn require_token<'a>(&'a self, name: &'a str) -> Result<&'a str, FbxTryFromReason> {
        let attr = self
            .extract(name)
            .ok_or_else(|| FbxTryFromReason::MissingAttribute {
                name: name.to_string(),
            })?;
        let tok =
            attr.get_tokens()
                .first()
                .ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
                    name: name.to_string(),
                    detail: "missing value token".into(),
                })?;
        Ok(tok.as_str())
    }
    fn require_token_case_insensitive(&self, name: &str) -> Result<&str, FbxTryFromReason> {
        let attr =
            self.extract_case_insensitive(name)
                .ok_or(FbxTryFromReason::MissingAttribute {
                    name: name.to_string(),
                })?;
        let tok =
            attr.get_tokens()
                .first()
                .ok_or_else(|| FbxTryFromReason::InvalidAttributeFormat {
                    name: name.to_string(),
                    detail: "missing value token".into(),
                })?;
        Ok(tok.as_str())
    }
    fn optional_token<'a>(&'a self, name: &'a str) -> Result<Option<&'a str>, FbxTryFromReason> {
        let Some(attr) = self.extract(name) else {
            return Ok(None);
        };
        Ok(attr.get_tokens().first().map(|s| s.as_str()))
    }
    fn optional_tokens<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a [String]>, FbxTryFromReason> {
        let Some(attr) = self.extract(name) else {
            return Ok(None);
        };
        Ok(Some(attr.get_tokens()))
    }
    fn optional_tokens_case_insensitive<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a [String]>, FbxTryFromReason> {
        let Some(attr) = self.extract_case_insensitive(name) else {
            return Ok(None);
        };
        Ok(Some(attr.get_tokens()))
    }
    fn optional_token_case_insensitive<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<Option<&'a str>, FbxTryFromReason> {
        let Some(attr) = self.extract_case_insensitive(name) else {
            return Ok(None);
        };
        Ok(attr.get_tokens().first().map(|s| s.as_str()))
    }
}

// --- Parsed multi-token attributes (ModelUVTranslation, Cropping, …) ---------------------------

fn parse_f32_token(attr_name: &str, tok: &str) -> Result<f32, FbxTryFromReason> {
    tok.trim().parse().map_err(|e: std::num::ParseFloatError| {
        FbxTryFromReason::InvalidAttributeFormat {
            name: attr_name.to_string(),
            detail: format!("invalid float token {tok:?}: {e}"),
        }
    })
}

fn parse_i32_token(attr_name: &str, tok: &str) -> Result<i32, FbxTryFromReason> {
    tok.trim().parse().map_err(|e: std::num::ParseIntError| {
        FbxTryFromReason::InvalidAttributeFormat {
            name: attr_name.to_string(),
            detail: format!("invalid int token {tok:?}: {e}"),
        }
    })
}

/// Optional parsing of typed token lists on top of [`AttrExtractor`].
pub trait AttrExtractorParseExt {
    /// First two value tokens as `f32` (e.g. `ModelUVTranslation`). `None` if the attribute is absent.
    fn optional_two_f32(&self, name: &str) -> Result<Option<[f32; 2]>, FbxTryFromReason>;
    fn optional_two_f32_case_insensitive(
        &self,
        name: &str,
    ) -> Result<Option<[f32; 2]>, FbxTryFromReason>;
    /// First four value tokens as `i32` (e.g. `Cropping`). `None` if the attribute is absent.
    fn optional_four_i32(&self, name: &str) -> Result<Option<[i32; 4]>, FbxTryFromReason>;
    fn optional_four_i32_case_insensitive(
        &self,
        name: &str,
    ) -> Result<Option<[i32; 4]>, FbxTryFromReason>;
}

impl<T: AttrExtractor> AttrExtractorParseExt for T {
    fn optional_two_f32(&self, name: &str) -> Result<Option<[f32; 2]>, FbxTryFromReason> {
        let Some(attr) = self.extract(name) else {
            return Ok(None);
        };
        let t = attr.get_tokens();
        if t.len() < 2 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: name.to_string(),
                detail: format!(
                    "expected at least 2 float tokens (e.g. ModelUVTranslation), got {}",
                    t.len()
                ),
            });
        }
        Ok(Some([
            parse_f32_token(name, &t[0])?,
            parse_f32_token(name, &t[1])?,
        ]))
    }

    fn optional_two_f32_case_insensitive(
        &self,
        name: &str,
    ) -> Result<Option<[f32; 2]>, FbxTryFromReason> {
        let Some(attr) = self.extract_case_insensitive(name) else {
            return Ok(None);
        };
        let t = attr.get_tokens();
        if t.len() < 2 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: name.to_string(),
                detail: format!(
                    "expected at least 2 float tokens (e.g. ModelUVTranslation), got {}",
                    t.len()
                ),
            });
        }
        Ok(Some([
            parse_f32_token(name, &t[0])?,
            parse_f32_token(name, &t[1])?,
        ]))
    }

    fn optional_four_i32(&self, name: &str) -> Result<Option<[i32; 4]>, FbxTryFromReason> {
        let Some(attr) = self.extract(name) else {
            return Ok(None);
        };
        let t = attr.get_tokens();
        if t.len() < 4 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: name.to_string(),
                detail: format!("expected 4 int tokens (Cropping), got {}", t.len()),
            });
        }
        Ok(Some([
            parse_i32_token(name, &t[0])?,
            parse_i32_token(name, &t[1])?,
            parse_i32_token(name, &t[2])?,
            parse_i32_token(name, &t[3])?,
        ]))
    }

    fn optional_four_i32_case_insensitive(
        &self,
        name: &str,
    ) -> Result<Option<[i32; 4]>, FbxTryFromReason> {
        let Some(attr) = self.extract_case_insensitive(name) else {
            return Ok(None);
        };
        let t = attr.get_tokens();
        if t.len() < 4 {
            return Err(FbxTryFromReason::InvalidAttributeFormat {
                name: name.to_string(),
                detail: format!("expected 4 int tokens (Cropping), got {}", t.len()),
            });
        }
        Ok(Some([
            parse_i32_token(name, &t[0])?,
            parse_i32_token(name, &t[1])?,
            parse_i32_token(name, &t[2])?,
            parse_i32_token(name, &t[3])?,
        ]))
    }
}

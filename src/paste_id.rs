use std::{borrow::Cow, path::PathBuf};

use random_word::Lang;
use rand::RngExt;
use rocket::request::FromParam;

use crate::storage::storage;

/// A unique id for a paste. Composed of 4 random english words, and separated by underscores.
#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {

    /// Generates a new paste id with a randomly generated number of words between 1 and 8.
    pub fn new() -> PasteId<'a> {
        let mut id = String::new();

        let words = rand::rng().random_range(1..8);

        for _ in 1..words {
            id = id + random_word::get(Lang::En) + "_";
        }

        // Remove the last _
        let (id, _) = id.split_at(id.len() - 1);

        PasteId(Cow::Owned(String::from(id)))
    }

    // Returns the path in the storage folder for the paste.
    pub fn path(&self) -> PathBuf {
        storage().join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_alphabetic() || c.eq_ignore_ascii_case(&'_'))
        .then(|| PasteId(param.into()))
        .ok_or(param)
    }
}
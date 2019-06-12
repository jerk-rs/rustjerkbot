use failure::Error;
use liquid::{value::liquid_value, ParserBuilder, Template};
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize)]
struct RawTemplateStore {
    found: String,
    not_found: String,
    new: Vec<String>,
    banned_users: Vec<i64>,
}

pub struct TemplateStore {
    found: Template,
    not_found: String,
    new: Vec<Template>,
    banned_users: Vec<i64>,
}

impl TemplateStore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let parser = ParserBuilder::with_liquid().build()?;
        let raw_store: RawTemplateStore = serde_yaml::from_slice(&buf)?;
        let found = parser.parse(&raw_store.found)?;
        let not_found = raw_store.not_found;
        let mut new = Vec::new();
        for i in raw_store.new {
            new.push(parser.parse(&i)?);
        }
        Ok(Self {
            found,
            not_found,
            new,
            banned_users: raw_store.banned_users,
        })
    }

    pub fn is_user_banned(&self, user_id: i64) -> bool {
        self.banned_users.iter().any(|x| *x == user_id)
    }

    pub fn render_template(
        &self,
        kind: TemplateKind,
        first: &str,
        second: &str,
    ) -> Result<Option<String>, Error> {
        let vars = liquid_value!({
            "first": first,
            "second": second
        })
        .into_object()
        .expect("Can not convert template vars into object");
        Ok(
            if let Some(template) = match kind {
                TemplateKind::Found => Some(&self.found),
                TemplateKind::New => self.new.choose(&mut thread_rng()),
            } {
                Some(template.render(&vars)?.trim().to_string())
            } else {
                None
            },
        )
    }

    pub fn get_not_found_message(&self) -> &str {
        &self.not_found
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TemplateKind {
    Found,
    New,
}

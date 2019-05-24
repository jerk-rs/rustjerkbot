use failure::{Error, Fail};
use liquid::{value::liquid_value, Parser, ParserBuilder, Template};
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::File, io::Read, path::Path, sync::Arc};

type Templates = Vec<Arc<Template>>;

/// Store for `.liquid` templates
#[derive(Clone)]
pub struct TemplateStore {
    parser: Parser,
    found: Templates,
    not_found: Vec<String>,
    new: Templates,
}

impl TemplateStore {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            parser: ParserBuilder::with_liquid().build()?,
            found: Templates::new(),
            not_found: Vec::new(),
            new: Templates::new(),
        })
    }

    /// Loads a list of templates from a file
    ///
    /// Templates must be separated by an empty line
    pub fn load_file<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let mut template = String::new();
        let mut kind: Option<TemplateKind> = None;

        macro_rules! parse_template {
            () => {
                self.parser.parse(&template).map(Arc::new)?
            };
        }

        macro_rules! push_template {
            () => {{
                match kind {
                    Some(TemplateKind::New) => self.new.push(parse_template!()),
                    Some(TemplateKind::Found) => self.found.push(parse_template!()),
                    Some(TemplateKind::NotFound) => self.not_found.push(template),
                    None => return Err(TemplateError::NoSection.into()),
                }
            }};
        }

        for mut line in buf.lines() {
            line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                kind = Some(match line {
                    "[found]" => TemplateKind::Found,
                    "[not-found]" => TemplateKind::NotFound,
                    "[new]" => TemplateKind::New,
                    _ => return Err(TemplateError::UnknownSection(String::from(line)).into()),
                });
                continue;
            }

            if line.is_empty() && !template.is_empty() {
                push_template!();
                template = String::new();
            } else {
                template += "\n";
                template += line;
            }
        }
        if !template.is_empty() {
            push_template!();
        }
        Ok(())
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
        let store = match kind {
            TemplateKind::Found => &self.found,
            TemplateKind::New => &self.new,
            TemplateKind::NotFound => return Err(TemplateError::UnexpectedKind(kind).into()),
        };
        match store.choose(&mut thread_rng()) {
            Some(template) => Ok(Some(template.render(&vars)?.trim().to_string())),
            None => Ok(None),
        }
    }

    pub fn get_not_found_message(&self) -> String {
        self.not_found
            .choose(&mut thread_rng())
            .cloned()
            .unwrap_or(String::from("Pair not found"))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TemplateKind {
    NotFound,
    Found,
    New,
}

#[derive(Debug, Fail)]
enum TemplateError {
    #[fail(display = "Template is not in a section")]
    NoSection,
    #[fail(display = "Unknown section: {}", _0)]
    UnknownSection(String),
    #[fail(display = "Can not render template for kind {:?}", _0)]
    UnexpectedKind(TemplateKind),
}

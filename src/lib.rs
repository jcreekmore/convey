extern crate failure;
extern crate serde;
extern crate termcolor;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Write;

pub fn new() -> Output {
    Output::default()
}

#[derive(Default)]
pub struct Output {
    targets: Vec<Target>,
}

impl Output {
    pub fn add(mut self, target: Target) -> Self {
        self.targets.push(target);
        self
    }
}

pub enum Target {
    Human(human::Formatter),
    Json(json::Formatter),
}

impl Output {
    pub fn print<O: RenderOutput>(&mut self, item: O) -> Result<(), ::failure::Error> {
        for target in &mut self.targets {
            match target {
                Target::Human(fmt) => {
                    item.render_for_humans(fmt)?;
                    fmt.writer.write(b"\n")?;
                }
                Target::Json(fmt) => {
                    item.render_json(fmt)?;
                    fmt.writer.write(b"\n")?;
                }
            }
        }

        Ok(())
    }
}

pub trait RenderOutput {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), ::failure::Error>;
    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), ::failure::Error>;
}

impl<'a, T> RenderOutput for &'a T
where
    T: RenderOutput,
{
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), ::failure::Error> {
        (*self).render_for_humans(fmt)
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), ::failure::Error> {
        (*self).render_json(fmt)
    }
}

pub mod components;
pub mod human;
pub mod json;
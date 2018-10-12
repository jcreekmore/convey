use {human, json, Error, Render};

/// Render some text
pub fn text<T: AsRef<str>>(input: T) -> Text {
    Text(input.as_ref().to_string())
}

/// Render a newline
///
/// This is just a shorthand for calling `text`.
pub fn newline() -> Text {
    text("\n")
}

#[derive(Clone, Serialize)]
pub struct Text(String);

impl Render for Text {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.write(self.0.as_bytes())?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        fmt.write(self)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::text;
    use {human, json, Render};

    proptest! {
        #[test]
        fn renders_text_exactly(s in "\\PC") {
            let item = text(&s);

            let human_output = human::test();
            item.render_for_humans(&mut human_output.formatter()).unwrap();
            prop_assert_eq!(&human_output.to_string(), &s);

            let json = json::test();
            item.render_json(&mut json.formatter()).unwrap();
            prop_assert_eq!(json.to_string(), json!(s).to_string() + "\n");
        }
    }
}

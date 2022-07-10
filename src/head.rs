struct HeadBuilder {
    title: Option<String>,
}

/// The type-safe interface for the `<head>` tag.
pub trait Head {
    /// Add a `<title>` to the head.
    fn title(&mut self, title: &str);
}

impl HeadBuilder {
    fn new() -> Self {
        Self {
            title: None,
        }
    }

    fn build(self) -> String {
        let mut output = String::from("<head>");
        if let Some(title) = self.title {
            output += format!("<title>{}</title>", title).as_str();
        }
        output += "</head>";
        output
    }
}

impl Head for HeadBuilder {
    fn title(&mut self, title: &str) {
        self.title = Some(title.to_owned());
    }
}

/// If you just need a `<head>` tag, use this function.
pub fn head(content: &dyn Fn(&mut dyn Head)) -> String {
    let mut hb = HeadBuilder::new();
    content(&mut hb);
    hb.build()
}

#[cfg(test)]
mod tests {
    use crate::head;

    #[test]
    fn empty_head() {
        let tag = head(&|_| {});
        assert_eq!(tag, "<head></head>");
    }

    #[test]
    fn title() {
        let tag = head(&|h| { h.title("Hello"); });
        assert_eq!(tag, "<head><title>Hello</title></head>");
    }
}

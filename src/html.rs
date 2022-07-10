use crate::{body, Body, head, Head};

struct HtmlBuilder {
    head: Option<String>,
    body: Option<String>,
}

/// The type-safe interface for the `<html>` tag.
pub trait Html {
    /// Add a `<head>` to the content.
    fn head(&mut self, content: &dyn Fn(&mut dyn Head));

    /// Add a `<body>` to the content.
    fn body(&mut self, content: &dyn Fn(&mut dyn Body));
}

impl HtmlBuilder {
    fn new() -> Self {
        HtmlBuilder {
            head: None,
            body: None,
        }
    }

    fn build(self) -> String {
        let mut output = String::from("<!DOCTYPE html><html>");
        if let Some(head) = self.head {
            output += head.as_str();
        }
        if let Some(body) = self.body {
            output += body.as_str();
        }
        output += "</html>";
        output
    }
}

impl Html for HtmlBuilder {
    fn head(&mut self, content: &dyn Fn(&mut dyn Head)) {
        self.head = Some(head(&content));
    }

    fn body(&mut self, content: &dyn Fn(&mut dyn Body)) {
        self.body = Some(body(&content));
    }
}

/// The main entry point for HTML generation.
///
/// # Example
///
/// ```rust
/// # use htmlgen::html;
/// let doc = html(&|_| {});
/// assert_eq!(doc, "<!DOCTYPE html><html></html>");
/// ```
///
pub fn html(content: &dyn Fn(&mut dyn Html)) -> String {
    let mut hb = HtmlBuilder::new();
    content(&mut hb);
    hb.build()
}

#[cfg(test)]
mod tests {
    use crate::html;

    #[test]
    fn empty_document() {
        let doc = html(&|_| {});
        assert_eq!(doc, "<!DOCTYPE html><html></html>");
    }

    #[test]
    fn empty_head() {
        let doc = html(&|h| { h.head(&|_| {}); });
        assert_eq!(doc, "<!DOCTYPE html><html><head></head></html>");
    }

    #[test]
    fn empty_body() {
        let doc = html(&|h| { h.body(&|_| {}); });
        assert_eq!(doc, "<!DOCTYPE html><html><body></body></html>");
    }

    #[test]
    fn empty_head_and_body() {
        let doc = html(&|h| {
            h.head(&|_| {});
            h.body(&|_| {});
        });
        assert_eq!(doc, "<!DOCTYPE html><html><head></head><body></body></html>");
    }
}

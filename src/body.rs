struct BodyBuilder {}

/// The type-safe interface for the `<body>` tag.
pub trait Body {}

impl BodyBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> String {
        String::from("<body></body>")
    }
}

impl Body for BodyBuilder {}

/// If you just need a `<body>` tag, use this function.
pub fn body(content: &dyn Fn(&mut dyn Body)) -> String {
    let mut bb = BodyBuilder::new();
    content(&mut bb);
    bb.build()
}

#[cfg(test)]
mod tests {
    use crate::body::body;

    #[test]
    fn empty_body() {
        let tag = body(&|_| {});
        assert_eq!(tag, "<body></body>");
    }
}

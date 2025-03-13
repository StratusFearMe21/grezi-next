pub struct MathMLDoc<'a> {
    tree: roxmltree::Document<'a>,
}

impl<'a> MathMLDoc<'a> {
    pub fn new(document: &'a str) -> Result<MathMLDoc<'a>, roxmltree::Error> {
        let tree = roxmltree::Document::parse(document)?;
        if tree.root().tag_name().name() != "math" {
            return Err(roxmltree::Error::InvalidName(
                tree.text_pos_at(tree.root().range().start),
            ));
        }
        Ok(Self { tree })
    }

    // pub fn resolve(&mut self, buffers: buffers) {}
}

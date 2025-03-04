use std::{borrow::Cow, io, sync::Arc};

use ropey::RopeSlice;
use smallvec::SmallVec;
use smart_default::SmartDefault;
use syntax_highlighting::format_highlighted;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use crate::{
    parse::{
        cursor::GrzCursorGuard,
        error::{ErrsWithSource, ParseError},
    },
    text::{Attrs, TextParagraph, TextSection},
};

mod dark_plus_theme;
mod format_djot;
mod syntax_highlighting;

#[derive(Default)]
pub struct TextJobParams<'a> {
    pub default_attrs: Attrs,
    pub default_font_size: f32,
    pub language: StringLiteral<'a>,
    pub value: StringLiteral<'a>,
}

impl From<TextJobParams<'_>> for SmallVec<[TextSection; 1]> {
    fn from(val: TextJobParams) -> Self {
        if !val.language.is_empty() {
            let mut text_job = val.new_paragraph();
            let value: Cow<'_, str> = val.value.into();
            let language: Cow<'_, str> = val.language.into();
            if !format_highlighted(
                value.as_ref(),
                language.as_ref(),
                &val.default_attrs,
                &mut text_job,
            ) {
                text_job
                    .rich_text
                    .push((value.into(), val.default_attrs.clone()));
            }
            smallvec::smallvec![TextSection::Paragraph(text_job)]
        } else {
            val.format_djot()
        }
    }
}

impl TextJobParams<'_> {
    fn new_paragraph(&self) -> TextParagraph {
        TextParagraph {
            font_size: self.default_font_size,
            ..Default::default()
        }
    }
}

#[derive(SmartDefault, Clone)]
pub enum StringLiteral<'a> {
    FullContent(RopeSlice<'a>),
    #[default]
    Escaped(String),
}

impl From<StringLiteral<'_>> for Arc<str> {
    fn from(value: StringLiteral<'_>) -> Self {
        match value {
            StringLiteral::FullContent(content) => content.to_string().into(),
            StringLiteral::Escaped(escaped) => escaped.into(),
        }
    }
}

impl<'a> From<StringLiteral<'a>> for Cow<'a, str> {
    fn from(value: StringLiteral<'a>) -> Self {
        match value {
            StringLiteral::FullContent(content) => content.into(),
            StringLiteral::Escaped(escaped) => Cow::Owned(escaped),
        }
    }
}

impl<'a> StringLiteral<'a> {
    pub fn as_rope_slice(&'a self) -> RopeSlice<'a> {
        match self {
            Self::FullContent(content) => *content,
            Self::Escaped(escaped) => RopeSlice::from(escaped.as_str()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::FullContent(content) => content.len_bytes() == 0,
            Self::Escaped(escaped) => escaped.is_empty(),
        }
    }

    #[instrument(skip_all, fields(source = %cursor.parent_source()?))]
    pub fn parse_from_cursor(
        mut cursor: GrzCursorGuard<'a, '_>,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<StringLiteral<'a>> {
        use std::fmt::Write;

        let initial_rope_slice = match NodeKind::from(cursor.node().kind_id()) {
            NodeKind::SymStringContent | NodeKind::SymRawStringContent => cursor.rope_slice()?,
            NodeKind::SymEscapeSequence => cursor.rope_slice()?.slice(1..),
            kind => {
                errors.append_error(
                    ParseError::BadNode(
                        cursor.char_range()?,
                        kind,
                        "Expected string content, an escape",
                    ),
                    cursor.error_info(),
                );
                RopeSlice::from("")
            }
        };

        if !cursor.goto_next_sibling()? {
            return Ok(StringLiteral::FullContent(initial_rope_slice));
        }

        let mut result = initial_rope_slice.to_string();

        loop {
            match NodeKind::from(cursor.node().kind_id()) {
                NodeKind::SymStringContent => result
                    .write_fmt(format_args!("{}", cursor.rope_slice()?))
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                NodeKind::SymEscapeSequence => result
                    .write_fmt(format_args!("{}", cursor.rope_slice()?.slice(1..)))
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                kind => errors.append_error(
                    ParseError::BadNode(
                        cursor.char_range()?,
                        kind,
                        "Expected string content, an escape",
                    ),
                    cursor.error_info(),
                ),
            }

            if !cursor.goto_next_sibling()? {
                break;
            }
        }

        Ok(StringLiteral::Escaped(result))
    }
}

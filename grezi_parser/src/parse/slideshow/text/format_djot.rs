use std::borrow::Cow;

use ecolor::Color32;
use jotdown::{Container, ListKind, OrderedListNumbering, OrderedListStyle, Parser};
use nominals::{Decimal, LetterLower, LetterUpper, Nominal, RomanLower, RomanUpper};
use smallvec::SmallVec;

use crate::{
    parse::slideshow::{
        actions::HIGHLIGHT_COLOR_DEFAULT, text::syntax_highlighting::format_highlighted,
    },
    text::{Family, Style, TextParagraph, TextSection, Weight},
};

use super::TextJobParams;

impl TextJobParams<'_> {
    pub fn format_djot(mut self) -> SmallVec<[TextSection; 1]> {
        // TODO: implement parsing in `jotdown` that
        // doesn't require re-allocating the String
        let value: Cow<'_, str> = std::mem::take(&mut self.value).into();
        let mut parser = Parser::new(value.as_ref());

        let mut job = SmallVec::new();
        while let Some(event) = parser.next() {
            if let jotdown::Event::Start(container, _attributes) = event {
                if container.is_block_container() {
                    self.format_block_container(container, &mut parser, &mut job);
                } else if container.is_block() {
                    let mut paragraph = self.new_paragraph();
                    paragraph.format_block(container, &mut parser, &self);
                    job.push(TextSection::Paragraph(paragraph));
                }
            }
        }
        job
    }

    pub fn format_block_container(
        &mut self,
        container: Container<'_>,
        parser: &mut Parser,
        text_job: &mut SmallVec<[TextSection; 1]>,
    ) {
        match container {
            // Unsupported
            Container::Footnote { .. }
            | Container::Table
            | Container::TableRow { .. }
            | Container::Section { .. }
            | Container::Div { .. }
            | Container::TaskListItem { .. }
            | Container::DescriptionList
            | Container::DescriptionDetails => {}
            Container::Blockquote => {
                let block = format_textjob_djot(parser, Some(container), self);
                text_job.push(TextSection::Blockquote(block.into_vec()));
            }
            Container::ListItem => {}
            Container::List { kind, .. } => {
                let mut list = Vec::new();
                let mut numberer = ListNumberer::new(kind);
                while let Some(event) = parser.next() {
                    match event {
                        jotdown::Event::Start(c @ Container::ListItem, _attributes) => {
                            let number = numberer.next();
                            let list_item = format_textjob_djot(parser, Some(c), self);
                            list.push((
                                TextParagraph {
                                    rich_text: smallvec::smallvec![(
                                        number,
                                        self.default_attrs.clone()
                                    )],
                                    font_size: self.default_font_size,
                                },
                                list_item.into_vec(),
                            ));
                        }
                        jotdown::Event::End(Container::List { .. }) => break,
                        _ => {}
                    }
                }
                text_job.push(TextSection::List(list));
            }
            _ => {}
        }
    }
}

fn format_textjob_djot(
    parser: &mut Parser,
    container: Option<Container<'_>>,
    params: &mut TextJobParams,
) -> SmallVec<[TextSection; 1]> {
    let mut job = SmallVec::new();
    while let Some(event) = parser.next() {
        match event {
            jotdown::Event::Start(container, _attributes) => {
                if container.is_block_container() {
                    params.format_block_container(container, parser, &mut job);
                } else if container.is_block() {
                    let mut paragraph = params.new_paragraph();
                    paragraph.format_block(container, parser, params);
                    job.push(TextSection::Paragraph(paragraph));
                }
            }
            jotdown::Event::End(c) => {
                if container
                    .as_ref()
                    .map(|container| c.eq(container))
                    .unwrap_or_default()
                {
                    break;
                }
            }
            _ => {}
        }
    }
    job
}

impl TextParagraph {
    pub fn format_block(
        &mut self,
        container: Container<'_>,
        parser: &mut Parser<'_>,
        params: &TextJobParams,
    ) {
        let mut attrs = params.default_attrs.clone();
        match container {
            // Unsupported
            Container::TableCell { .. }
            | Container::Caption
            | Container::DescriptionTerm
            | Container::LinkDefinition { .. }
            | Container::RawBlock { .. } => {}

            Container::Heading { level, .. } => {
                let l = match level {
                    1 => 2.0,
                    2 => 1.5,
                    3 => 1.17,
                    4 => 1.0,
                    5 => 0.83,
                    _ => 0.67,
                };
                self.font_size *= l;
            }
            Container::CodeBlock { language } => {
                attrs.family = Family::Monospace;
                if !language.is_empty() {
                    match parser.next() {
                        Some(jotdown::Event::Str(code)) => {
                            if format_highlighted(code.trim(), language, &attrs, self) {
                                return;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Container::Paragraph => {}

            _ => {}
        }
        macro_rules! new_rich_text {
            () => {
                self.rich_text
                    .push((smartstring::alias::String::new(), attrs.clone()));
            };
        }
        macro_rules! get_last_rich_text {
            () => {
                if let Some(lm) = self.rich_text.last_mut() {
                    lm
                } else {
                    new_rich_text!();
                    self.rich_text.last_mut().unwrap()
                }
            };
        }
        while let Some(event) = parser.next() {
            match event {
                jotdown::Event::LeftSingleQuote => {
                    get_last_rich_text!().0.push('‘');
                }
                jotdown::Event::LeftDoubleQuote => {
                    get_last_rich_text!().0.push('“');
                }
                jotdown::Event::RightSingleQuote => {
                    get_last_rich_text!().0.push('’');
                }
                jotdown::Event::RightDoubleQuote => {
                    get_last_rich_text!().0.push('”');
                }
                jotdown::Event::Ellipsis => {
                    get_last_rich_text!().0.push('…');
                }
                jotdown::Event::EmDash => {
                    get_last_rich_text!().0.push('—');
                }
                jotdown::Event::EnDash => {
                    get_last_rich_text!().0.push('–');
                }
                jotdown::Event::Softbreak | jotdown::Event::NonBreakingSpace => {
                    get_last_rich_text!().0.push(' ');
                }
                jotdown::Event::Hardbreak | jotdown::Event::ThematicBreak(_) => {
                    get_last_rich_text!().0.push('\n');
                }
                jotdown::Event::Str(s) => get_last_rich_text!().0.push_str(s.as_ref()),
                jotdown::Event::Symbol(symbol) => {
                    if let Some(emoji) = emojis::get_by_shortcode(symbol.as_ref()) {
                        get_last_rich_text!().0.push_str(emoji.as_str());
                    }
                }
                jotdown::Event::Start(container, attributes) => match container {
                    // Unsupported
                    Container::Span
                    | Container::Link(..)
                    | Container::Math { .. }
                    | Container::Image(..)
                    | Container::RawInline { .. }
                    | Container::Subscript
                    | Container::Superscript => {}
                    Container::Verbatim => {
                        attrs.family = Family::Monospace;
                        if let Some(lang) = attributes.get_value("lang") {
                            match parser.next() {
                                Some(jotdown::Event::Str(code)) => {
                                    if format_highlighted(
                                        code.as_ref(),
                                        lang.parts()
                                            .collect::<smartstring::alias::String>()
                                            .as_str(),
                                        &attrs,
                                        self,
                                    ) {
                                        attrs.family = params.default_attrs.family.clone();
                                        continue;
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        new_rich_text!();
                    }
                    Container::Insert => {
                        attrs.color = Color32::GREEN;
                        new_rich_text!();
                    }
                    Container::Delete => {
                        attrs.color = Color32::RED;
                        new_rich_text!();
                    }
                    Container::Mark => {
                        attrs.color = HIGHLIGHT_COLOR_DEFAULT;
                        new_rich_text!();
                    }
                    Container::Strong => {
                        attrs.weight = Weight::BOLD;
                        new_rich_text!();
                    }
                    Container::Emphasis => {
                        attrs.style = Style::Italic;
                        new_rich_text!();
                    }
                    _ => {}
                },
                jotdown::Event::End(c) => {
                    if container == c {
                        if get_last_rich_text!().0.is_empty() {
                            self.rich_text.pop();
                        }
                        break;
                    }

                    match c {
                        // Unsupported
                        Container::Span
                        | Container::Link(..)
                        | Container::Math { .. }
                        | Container::Image(..)
                        | Container::RawInline { .. }
                        | Container::Subscript
                        | Container::Superscript => {}
                        Container::Verbatim => {
                            attrs.family = params.default_attrs.family.clone();
                            new_rich_text!();
                        }
                        Container::Insert | Container::Delete | Container::Mark => {
                            attrs.color = params.default_attrs.color;
                            new_rich_text!();
                        }
                        Container::Strong => {
                            attrs.weight = params.default_attrs.weight;
                            new_rich_text!();
                        }
                        Container::Emphasis => {
                            attrs.style = params.default_attrs.style;
                            new_rich_text!();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

struct ListNumberer {
    list_kind: ListKind,
    at: u64,
}

impl ListNumberer {
    pub fn new(list_kind: ListKind) -> Self {
        Self { list_kind, at: 0 }
    }
}

impl ListNumberer {
    pub fn next(&mut self) -> smartstring::alias::String {
        let result = match self.list_kind {
            ListKind::Unordered(_) | ListKind::Task(_) => return "•".into(),
            ListKind::Ordered {
                numbering,
                style,
                start,
            } => {
                let number = start + self.at;
                let nominal = match numbering {
                    OrderedListNumbering::Decimal => number.to_nominal(&Decimal),
                    OrderedListNumbering::AlphaLower => number.to_nominal(&LetterLower),
                    OrderedListNumbering::AlphaUpper => number.to_nominal(&LetterUpper),
                    OrderedListNumbering::RomanLower => number.to_nominal(&RomanLower),
                    OrderedListNumbering::RomanUpper => number.to_nominal(&RomanUpper),
                };

                let mut result = smartstring::alias::String::new();

                match style {
                    OrderedListStyle::Period => {
                        result.push_str(&nominal);
                        result.push('.');
                    }
                    OrderedListStyle::Paren => {
                        result.push_str(&nominal);
                        result.push(')');
                    }
                    OrderedListStyle::ParenParen => {
                        result.push('(');
                        result.push_str(&nominal);
                        result.push(')');
                    }
                }

                result
            }
        };
        self.at += 1;
        result
    }
}

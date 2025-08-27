//! Boilerplate for serializing and deserializing font data into `.slideshow` files.

use std::{borrow::Cow, hash::Hash, ops::Deref, sync::Arc};

use egui::{FontData, FontDefinitions, FontTweak};
use egui_glyphon::cosmic_text::{FontSystem, fontdb::Source};
use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeSeq};

pub struct FontRef(pub Arc<dyn AsRef<[u8]> + Send + Sync>);

impl Hash for FontRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for FontRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Serialize for FontRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.deref().as_ref())
    }
}

impl Eq for FontRef {}

pub struct IndexSliceSerializer<'a, T: Serialize>(pub &'a indexmap::set::Slice<T>);

impl<'a, T: Serialize> Serialize for IndexSliceSerializer<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for i in self.0.iter() {
            seq.serialize_element(i)?;
        }

        seq.end()
    }
}

pub struct FontSystemDeserializer(pub FontSystem, pub FontDefinitions);

impl<'de> Deserialize<'de> for FontSystemDeserializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (font_system, font_definitions): (FontSystem, FontDefinitions) =
            deserializer.deserialize_seq(DbVisitor)?;
        Ok(FontSystemDeserializer(font_system, font_definitions))
    }
}
struct DbVisitor;

impl<'de> Visitor<'de> for DbVisitor {
    type Value = (FontSystem, FontDefinitions);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A sequence of byte arrays containing fonts")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut definitions = FontDefinitions::empty();
        Ok((
            FontSystem::new_with_fonts(
                // If `next_element` fails we'll just get a "font not found" error
                // out of cosmic text. Or better, cosmic text will fall back to
                // a different font
                std::iter::from_fn(|| match seq.next_element::<Vec<u8>>() {
                    Ok(element) => element,
                    Err(e) => {
                        tracing::error!(error = ?e, "Deserializing font system failed");
                        None
                    }
                })
                .enumerate()
                .map(|(number, data)| {
                    let font_data = Arc::new(FontData {
                        font: Cow::Owned(data),
                        index: 0,
                        tweak: FontTweak::default(),
                    });

                    definitions
                        .font_data
                        .insert(number.to_string(), Arc::clone(&font_data));

                    definitions
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .and_modify(|e| e.push(number.to_string()))
                        .or_insert_with(|| vec![number.to_string()]);

                    Source::Binary(
                        Arc::clone(&font_data) as std::sync::Arc<dyn AsRef<[u8]> + Send + Sync>
                    )
                }),
            ),
            definitions,
        ))
    }
}

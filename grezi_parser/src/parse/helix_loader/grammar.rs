use phf::OrderedSet;
use std::path::PathBuf;
use tree_house::{InjectionLanguageMarker, LanguageConfig, LanguageLoader, highlighter::Highlight};
use tree_house_bindings::Grammar;

#[cfg(unix)]
const DYLIB_EXTENSION: &str = "so";

#[cfg(windows)]
const DYLIB_EXTENSION: &str = "dll";

pub struct HelixLanguageLoader {
    languages: boxcar::Vec<LanguageConfig>,
    names: &'static OrderedSet<&'static str>,
}

impl LanguageLoader for HelixLanguageLoader {
    fn language_for_marker(&self, marker: InjectionLanguageMarker) -> Option<tree_house::Language> {
        let name = match marker {
            InjectionLanguageMarker::Name(lang) => smartstring::alias::String::from(lang),
            InjectionLanguageMarker::Match(lang) => lang.chunks().collect(),
            InjectionLanguageMarker::Filename(_) => return None,
            InjectionLanguageMarker::Shebang(_) => return None,
        };

        let lang = get_highlight_configuration(name.as_str()).ok()?;
        lang.configure(|name| Some(Highlight::new(self.names.get_index(name)? as u32)));
        Some(tree_house::Language(self.languages.push(lang) as u32))
    }

    fn get_config(&self, lang: tree_house::Language) -> Option<&LanguageConfig> {
        self.languages.get(lang.0 as usize)
    }
}

impl HelixLanguageLoader {
    pub fn new(names: &'static OrderedSet<&'static str>) -> Self {
        Self {
            languages: boxcar::Vec::new(),
            names,
        }
    }
}

fn get_language(name: &str) -> Result<Grammar, tree_house_bindings::GrammarError> {
    let mut rel_library_path = PathBuf::new().join("grammars").join(name);
    rel_library_path.set_extension(DYLIB_EXTENSION);
    let library_path = super::runtime_file(&rel_library_path);

    unsafe { Grammar::new(name, &library_path) }
}

pub fn get_highlight_configuration(
    name: &str,
) -> Result<LanguageConfig, tree_house_bindings::GrammarError> {
    match name {
        "grz" => Ok(LanguageConfig::new(
            tree_sitter_grz::LANGUAGE.try_into().unwrap(),
            tree_sitter_grz::HIGHLIGHTS_QUERY,
            tree_sitter_grz::INJECTIONS_QUERY,
            "",
        )
        .unwrap()),
        _ => {
            let language = get_language(name)?;

            Ok(LanguageConfig::new(
                language,
                load_runtime_file(name, "highlights.scm")
                    .unwrap_or_default()
                    .as_str(),
                load_runtime_file(name, "injections.scm")
                    .unwrap_or_default()
                    .as_str(),
                load_runtime_file(name, "locals.scm")
                    .unwrap_or_default()
                    .as_str(),
            )
            .unwrap())
        }
    }
}

/// Gives the contents of a file from a language's `runtime/queries/<lang>`
/// directory
pub fn load_runtime_file(language: &str, filename: &str) -> Result<String, std::io::Error> {
    let path = super::runtime_file(&PathBuf::new().join("queries").join(language).join(filename));
    std::fs::read_to_string(path)
}

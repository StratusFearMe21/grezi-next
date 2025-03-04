use std::path::PathBuf;
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

#[cfg(unix)]
const DYLIB_EXTENSION: &str = "so";

#[cfg(windows)]
const DYLIB_EXTENSION: &str = "dll";

pub fn get_language(name: &str) -> Result<Language, libloading::Error> {
    use libloading::{Library, Symbol};
    let mut rel_library_path = PathBuf::new().join("grammars").join(name);
    rel_library_path.set_extension(DYLIB_EXTENSION);
    let library_path = super::runtime_file(&rel_library_path);

    let library = unsafe { Library::new(&library_path) }?;
    let language_fn_name = format!("tree_sitter_{}", name.replace('-', "_"));
    let language = unsafe {
        let language_fn: Symbol<unsafe extern "C" fn() -> Language> =
            library.get(language_fn_name.as_bytes())?;
        language_fn()
    };
    std::mem::forget(library);
    Ok(language)
}

pub fn get_highlight_configuration(
    name: &str,
) -> Result<HighlightConfiguration, libloading::Error> {
    let language = get_language(name)?;

    Ok(HighlightConfiguration::new(
        language,
        "language",
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

/// Gives the contents of a file from a language's `runtime/queries/<lang>`
/// directory
pub fn load_runtime_file(language: &str, filename: &str) -> Result<String, std::io::Error> {
    let path = super::runtime_file(&PathBuf::new().join("queries").join(language).join(filename));
    std::fs::read_to_string(path)
}

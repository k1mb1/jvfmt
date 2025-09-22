use fmtrunner::parser::LanguageProvider;
use fmtrunner::supported_extension::SupportedExtension;
use tree_sitter::Language;
use tree_sitter_java::LANGUAGE;


pub struct Java;

const JAVA_SUPPORTED_EXT: SupportedExtension = SupportedExtension::new(&["java"]);

impl LanguageProvider for Java {
    fn language() -> Language {
        LANGUAGE.into()
    }

    fn supported_extension() -> &'static SupportedExtension {
        &JAVA_SUPPORTED_EXT
    }
}

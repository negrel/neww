/// Enumeration of all supported script language.
#[derive(Debug, PartialEq)]
pub enum ScriptLang {
    Lua,
}

/// Script is an executable script.
#[derive(Debug)]
pub struct Script {
    pub lang: ScriptLang,
    pub source: String,
}

impl Script {
    pub fn new(lang: ScriptLang, source: String) -> Self {
        Self { lang, source }
    }
}

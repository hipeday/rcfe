use std::fmt::{Display, Formatter};
use fluent_templates::{langid, LanguageIdentifier, Loader};
use serde::{Serialize, Serializer};

pub const DEFAULT_LANG: Lang = Lang::ZhCn;
const I18N_NAMESPACE_SEPARATOR: &str = "-";

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "zh-CN",
    };
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Lang {
    ZhCn,
    EnUs,
}

impl Default for Lang {
    fn default() -> Self {
        DEFAULT_LANG
    }
}

impl Lang {
    pub fn local(&self) -> String {
        match self {
            Lang::ZhCn => String::from("简体中文"),
            Lang::EnUs => String::from("English"),
        }
    }
}

impl Serialize for Lang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Lang::ZhCn => serializer.serialize_str("zh-CN"),
            Lang::EnUs => serializer.serialize_str("en-US"),
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::ZhCn => write!(f, "zh-CN"),
            Lang::EnUs => write!(f, "en-US"),
        }
    }
}

impl Into<String> for Lang {
    fn into(self) -> String {
        match self {
            Lang::ZhCn => "zh-CN".to_string(),
            Lang::EnUs => "en-US".to_string(),
        }
    }
}

impl Into<&'static str> for Lang {
    fn into(self) -> &'static str {
        match self {
            Lang::ZhCn => "zh-CN",
            Lang::EnUs => "en-US",
        }
    }
}

impl Into<Lang> for String {
    fn into(self) -> Lang {
        match self.as_str() {
            "en-US" => Lang::EnUs,
            "zh-CN" => Lang::ZhCn,
            _ => Lang::ZhCn,
        }
    }
}

impl Into<LanguageIdentifier> for Lang {
    fn into(self) -> LanguageIdentifier {
        match self {
            Lang::ZhCn => langid!("zh-CN"),
            Lang::EnUs => langid!("en-US"),
        }
    }
}

pub fn t<ID: AsRef<str>>(lang: Lang, namespace: Option<&str>, text_id: ID) -> String {
    let lang_id: LanguageIdentifier = lang.into();

    let key = match namespace {
        Some(ns) => format!("{}{}{}", ns, I18N_NAMESPACE_SEPARATOR, text_id.as_ref()),
        None => text_id.as_ref().to_string(),
    };

    LOCALES.lookup(&lang_id, &key)
}

#[cfg(test)]
mod tests {
    use fluent_templates::Loader;
    use super::*;
    #[test]
    fn test_lang_into_string() {
        let lang: String = Lang::ZhCn.into();
        assert_eq!(lang, "zh-CN".to_string());
    }

    #[test]
    fn test_lang_into_str() {
        let lang: &str = Lang::ZhCn.into();
        assert_eq!(lang, "zh-CN");
    }

    #[test]
    fn test_lang_into_language_identifier() {
        let lang: LanguageIdentifier = Lang::ZhCn.into();
        assert_eq!(lang.to_string(), "zh-CN");
    }

    #[test]
    fn test_locale_lookup() {
        let lang: LanguageIdentifier = Lang::ZhCn.into();
        let greeting = LOCALES.lookup(&lang, "title");
        assert_eq!(greeting, "RCFE");
    }
}
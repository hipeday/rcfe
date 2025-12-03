use crate::{DEFAULT_LANG, Lang, t};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use serde::Serialize;

pub trait HtmlMetadata {
    fn namespace(&self) -> String;

    fn title(&self) -> Option<String> {
        Some(t(self.lang(), None, "title"))
    }
    fn subtitle(&self) -> Option<String> {
        Some(t(self.lang(), Some(self.namespace().as_str()), "subtitle"))
    }

    fn lang(&self) -> Lang {
        Lang::ZhCn
    }

    fn home_label(&self) -> String {
        t(self.lang(), None, "home_label")
    }

    fn current_path(&self) -> String;

    fn q(&self) -> Option<String> {
        None
    }
}

pub trait I18nMetadata: HtmlMetadata {
    fn languages(&self) -> Vec<Lang> {
        vec![Lang::ZhCn, Lang::EnUs]
    }

    fn selected_language(&self) -> Lang {
        self.lang()
    }

    fn default_language(&self) -> Lang {
        DEFAULT_LANG
    }

    fn select_language_label(&self) -> String {
        t(self.selected_language(), None, "select_language")
    }

    fn nt<ID: AsRef<str>>(&self, text_id: ID) -> String {
        t(self.lang(), Some(self.namespace().as_str()), text_id)
    }

    fn t<ID: AsRef<str>>(&self, text_id: ID) -> String {
        t(self.lang(), None, text_id)
    }

    fn href_with_lang(&self, lang: &Lang) -> String {
        let path = match *lang {
            Lang::ZhCn => {
                let path = format!("{}", self.current_path());
                if path.is_empty() {
                    "/".to_string()
                } else {
                    path
                }
            },
            _ => {
                format!("/{}{}", lang, self.current_path())
            },
        };
        match self.q() {
            None => path,
            Some(q) => {
                if q.is_empty() {
                    path
                } else {
                    format!("{}?{}", path, q)
                }
            },
        }
    }
}

pub struct HtmlTemplate<T>(T)
where
    T: Template + HtmlMetadata + Serialize + I18nMetadata;

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template + HtmlMetadata + Serialize + I18nMetadata,
{
    fn into_response(self) -> Response {
        let instance = self.0;
        match instance.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => Response::builder()
                .status(500)
                .body("Internal Server Error".to_string())
                .unwrap()
                .into_response(),
        }
    }
}

impl<T: HtmlMetadata + Serialize + Template + I18nMetadata> HtmlTemplate<T> {
    pub fn new(template: T) -> Self
    where
        T: Template + HtmlMetadata + Serialize + I18nMetadata,
    {
        HtmlTemplate(template)
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#![allow(unused_variables)]

use rust_i18n::t;
use regex::Regex;

use crate::database::tables;


#[derive(Copy, Clone)]
pub struct I18n<'a> {
    default_language: &'a str,
    escape_html: bool,
    language: &'a str,
}

impl<'a> Default for I18n<'a> {
    fn default() -> Self {
        Self {
            default_language: "en-GB",
            escape_html: true,
            language: "en-GB",
        }
    }
}

impl<'a> I18n<'a> {
    pub fn get_text(&self, key: &str, items: Vec<(&str, &str)>) -> String {
        let mut text = t!(&key, locale=self.get_language_code());

        let iter = items.clone().into_iter();
        for item in iter {
            text = text.replace(&["{", item.0, "}"].join(""), item.1);
        }

        let re = Regex::new(format!(r#"((\w+)-?(\w+)?).{}"#, key).as_str()).unwrap();
        if re.is_match(text.as_str()) {
            self.set_language(self.default_language);
            text = self.get_text(key, items);
            self.set_language(self.language);
        }

        text
    }

    pub fn get_language(self, language_code: &'a str) -> Self {
        Self {
            default_language: self.default_language,
            escape_html: self.escape_html,
            language: language_code,
        }

    }

    pub fn get_language_code(&self) -> &str {
        self.language
    }

    pub fn set_language(mut self, language_code: &'a str) {
        self.language = language_code;
    }
}

pub fn new() -> I18n<'static> {
    I18n {
        ..Default::default()
    }
}

pub fn from_user(id: i64) -> I18n<'static> {
    let user = tables::User::get(id).unwrap();

    let lang = new();
    lang.set_language(user.language.as_str());

    lang
}

pub fn from_group(id: i64) -> I18n<'static> {
    let group = tables::Group::get(id).unwrap();

    let lang = new();
    lang.set_language(group.language.as_str());

    lang
}

use std::{array, ops::Index};

// https://languagetool.org/http-api/swagger-ui/#!/default/post_words_add
// https://dev.languagetool.org/http-server.html
pub mod api;
pub mod check;
pub struct Response {
    pub software: Software,
    pub warnings: Warnings,
    pub language: Language,
    pub matches: Vec<Match>,
    // sentenceRanges
    pub sentence_ranges: Option<Vec<Vec<i32>>>,
    // extendedSentenceRanges
    pub extended_sentence_ranges: Option<Vec<ExtendedSentenceRange>>,
}

pub struct ExtendedSentenceRange {
    pub from: i32,
    pub to: i32,
    // detectedLanguages
    pub detected_languages: Vec<DetectedLanguage>,
}
pub struct DetectedLanguageZwei {
    pub language: String,
    pub rate: i32,
}
pub struct Software {
    pub name: String,
    pub version: String,
    // buildDate
    pub build_date: String,
    // apiVersion
    pub api_version: i32,
    pub premium: bool,
    // premiumHint
    pub premium_hint: String,
    pub status: String,
}
pub struct Language {
    pub name: String,
    pub code: String,
    // detectedLanguage
    pub detected_language: DetectedLanguage,
}
pub struct DetectedLanguage {
    pub name: Option<String>,
    pub code: Option<String>,
    pub confidence: Option<f32>,
    pub source: Option<String>,
    pub language: Option<String>,
    pub rate: Option<i32>,
}
pub struct Warnings {
    //incompleteResults
    pub incomplete_esults: bool,
}
pub struct Match {
    pub message: String,
    //shortMessage
    pub short_message: String,
    pub replacements: Vec<Replacements>,
    pub offset: u32,
    pub length: u32,
    pub context: Context,
    pub sentence: String,
    // type
    pub match_type: MatchType,
    pub rule: Rule,
    // ignoreForIncompleteSentence
    pub ignore_for_incomplete_sentence: bool,
    // contextForSureMatch
    pub context_for_sure_match: i32,
}
pub struct MatchType {
    // typeName
    pub type_name: String,
}
pub struct Replacements {
    pub value: String,
}

pub struct Context {
    pub text: String,
    pub offset: u32,
    pub length: u32,
}

pub struct Rule {
    pub id: String,
    // subId
    pub sub_id: Option<String>,
    pub description: String,
    // issueType
    pub issue_type: String,
    pub urls: Vec<Url>,
    pub category: Category,
    // isPremium
    pub is_premium: bool,
    pub confidence: f32,
}
pub struct Url {
    pub value: String,
}
pub struct Category {
    pub id: String,
    pub name: String,
}

#[no_mangle]
pub fn test_insert() {
    let mut x: [i32,10] = [10, 10];
    x[2] = 1;
    println!("{:?}", x)
}

fn main() {
    test_insert();
    println!("Hello, world!");
}

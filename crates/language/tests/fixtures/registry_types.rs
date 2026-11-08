//! Minimal trait context for the real shared macros in standalone rustc fixtures.
#![allow(dead_code)]
use std::{borrow::Cow, num::NonZeroU16};

pub trait Language {
  fn kind_to_id(&self, kind: &str) -> u16;
  fn field_to_id(&self, field: &str) -> Option<u16>;
  fn meta_var_char(&self) -> char {
    '$'
  }
  fn expando_char(&self) -> char;
  fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str>;
  fn build_pattern(&self, builder: &PatternBuilder) -> Result<Pattern, PatternError>;
}
pub trait LanguageExt {
  fn get_ts_language(&self) -> TSLanguage;
}
pub struct TSLanguage;
impl TSLanguage {
  pub fn id_for_node_kind(&self, _: &str, _: bool) -> u16 {
    0
  }
  pub fn field_id_for_name(&self, _: &str) -> Option<NonZeroU16> {
    None
  }
}
pub struct Pattern;
pub struct PatternError;
pub struct PatternBuilder;
pub struct StrDoc;
impl PatternBuilder {
  pub fn build(
    &self,
    _: impl FnOnce(&str) -> Result<StrDoc, PatternError>,
  ) -> Result<Pattern, PatternError> {
    Ok(Pattern)
  }
}
impl StrDoc {
  pub fn try_new<L>(_: &str, _: L) -> Result<Self, PatternError> {
    Ok(Self)
  }
}
pub trait Alias {
  const ALIAS: &'static [&'static str];
}
pub trait Deserializer<'de> {
  type Error;
}
pub trait Deserialize<'de>: Sized {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error>;
}
pub enum SupportLang {
  Probe,
}
pub mod parsers {
  pub fn unused() -> super::TSLanguage {
    super::TSLanguage
  }
}
pub mod registry_support {
  use super::*;
  pub fn prepare(_: Option<char>, query: &str) -> Cow<'_, str> {
    Cow::Borrowed(query)
  }
  pub fn validate_alias<'de, D: Deserializer<'de>>(
    _: D,
    _: &'static [&'static str],
  ) -> Result<(), D::Error> {
    Ok(())
  }
}

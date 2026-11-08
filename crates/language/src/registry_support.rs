use serde::{de,Deserializer};
use serde::de::Visitor;
use std::fmt;
use std::borrow::Cow;
struct AliasVisitor {
  aliases: &'static [&'static str],
}

impl Visitor<'_> for AliasVisitor {
  type Value = &'static str;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "one of {:?}", self.aliases)
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    self
      .aliases
      .iter()
      .copied()
      .find(|&a| v.eq_ignore_ascii_case(a))
      .ok_or_else(|| de::Error::invalid_value(de::Unexpected::Str(v), &self))
  }
}

fn pre_process_pattern(expando: char, query: &str) -> std::borrow::Cow<'_, str> {
  let mut ret = Vec::with_capacity(query.len());
  let mut dollar_count = 0;
  for c in query.chars() {
    if c == '$' {
      dollar_count += 1;
      continue;
    }
    let need_replace = matches!(c, 'A'..='Z' | '_') // $A or $$A or $$$A
      || dollar_count == 3; // anonymous multiple
    let sigil = if need_replace { expando } else { '$' };
    ret.extend(std::iter::repeat_n(sigil, dollar_count));
    dollar_count = 0;
    ret.push(c);
  }
  // trailing anonymous multiple
  let sigil = if dollar_count == 3 { expando } else { '$' };
  ret.extend(std::iter::repeat_n(sigil, dollar_count));
  std::borrow::Cow::Owned(ret.into_iter().collect())
}


pub(super) fn prepare(expando:Option<char>,query:&str)->Cow<'_,str> {match expando {Some(c)=>pre_process_pattern(c,query),None=>Cow::Borrowed(query)}}
pub(super) fn validate_alias<'de,D:Deserializer<'de>>(d:D,aliases:&'static[&'static str])->Result<(),D::Error>{d.deserialize_str(AliasVisitor{aliases})?;Ok(())}

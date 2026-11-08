//! Built-in programming languages, parser features and registry metadata.
//!
//! One registry declaration per language selects its parser, names, aliases,
//! extensions and preprocessing. Specialized behavior and tests remain in the
//! corresponding language modules. The registry can contain no enabled parsers.
//!
//! ```
//! # #[cfg(feature = "tree-sitter-rust")]
//! # {
//! use ast_grep_language::{LanguageExt, SupportLang};
//!
//! let lang: SupportLang = "rs".parse().unwrap();
//! let src = "fn foo() {}";
//! let root = lang.ast_grep(src);
//! let found = root.root().find_all("fn $FNAME() {}").next().unwrap();
//! assert_eq!(found.start_pos().line(), 0);
//! assert_eq!(found.text(), "fn foo() {}");
//! # }
//! ```

use ast_grep_core::matcher::{Pattern, PatternBuilder, PatternError};

use ast_grep_core::Node;
use ast_grep_core::meta_var::MetaVariable;
use ast_grep_core::tree_sitter::{StrDoc, TSLanguage, TSRange};
use ignore::types::{Types, TypesBuilder};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, de};
use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

pub use ast_grep_core::language::Language;
pub use ast_grep_core::tree_sitter::LanguageExt;

pub trait Alias: Display {
  const ALIAS: &'static [&'static str];
}

#[derive(Debug)]
pub enum SupportLangErr {
  LanguageNotSupported(String),
}

impl Display for SupportLangErr {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    use SupportLangErr::*;
    match self {
      LanguageNotSupported(lang) => write!(f, "{lang} is not supported!"),
    }
  }
}

impl std::error::Error for SupportLangErr {}

impl<'de> Deserialize<'de> for SupportLang {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(SupportLangVisitor)
  }
}

struct SupportLangVisitor;

impl Visitor<'_> for SupportLangVisitor {
  type Value = SupportLang;

  fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str("SupportLang")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    v.parse().map_err(de::Error::custom)
  }
}
/// Implements the language names and aliases.
impl FromStr for SupportLang {
  type Err = SupportLangErr;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for &lang in Self::all_langs() {
      for moniker in alias(lang) {
        if s.eq_ignore_ascii_case(moniker) {
          return Ok(lang);
        }
      }
    }
    Err(SupportLangErr::LanguageNotSupported(s.to_string()))
  }
}

/// Guess which programming language a file is written in
/// Adapt from `<https://github.com/Wilfred/difftastic/blob/master/src/parse/guess_language.rs>`
/// N.B do not confuse it with `FromStr` trait. This function is to guess language from file extension.
fn from_extension(path: &Path) -> Option<SupportLang> {
  let ext = path.extension()?.to_str()?;
  SupportLang::all_langs()
    .iter()
    .copied()
    .find(|&l| extensions(l).contains(&ext))
}

fn add_custom_file_type<'b>(
  builder: &'b mut TypesBuilder,
  file_type: &str,
  suffix_list: &[&str],
) -> &'b mut TypesBuilder {
  for suffix in suffix_list {
    let glob = format!("*.{suffix}");
    builder
      .add(file_type, &glob)
      .expect("file pattern must compile");
  }
  builder.select(file_type)
}

fn file_types(lang: SupportLang) -> Types {
  let mut builder = TypesBuilder::new();
  let exts = extensions(lang);
  let lang_name = lang.to_string();
  add_custom_file_type(&mut builder, &lang_name, exts);
  builder.build().expect("file type must be valid")
}

pub fn config_file_type() -> Types {
  let mut builder = TypesBuilder::new();
  let builder = add_custom_file_type(&mut builder, "yml", &["yml", "yaml"]);
  builder.build().expect("yaml type must be valid")
}

macro_rules! declare_module {
  ($feature:literal, none) => {};
  ($feature:literal, $module:ident) => {
    #[cfg(feature=$feature)]
    mod $module;
  };
}
macro_rules! declare_marker {
  ($feature:literal, $name:ident, $function:ident, $pre:expr, external) => {};
  ($feature:literal, $name:ident, $function:ident, $pre:expr, generated) => {
    #[cfg(feature=$feature)]
    #[derive(Clone, Copy, Debug)]
    pub struct $name;
    #[cfg(feature=$feature)]
    impl Language for $name {
      fn kind_to_id(&self, kind: &str) -> u16 {
        let _ = &kind;
        self.get_ts_language().id_for_node_kind(kind, true)
      }
      fn field_to_id(&self, field: &str) -> Option<u16> {
        let _ = &field;
        self
          .get_ts_language()
          .field_id_for_name(field)
          .map(|f| f.get())
      }
      fn expando_char(&self) -> char {
        ($pre).unwrap_or_else(|| self.meta_var_char())
      }
      fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str> {
        let _ = &query;
        registry_support::prepare($pre, query)
      }
      fn build_pattern(&self, builder: &PatternBuilder) -> Result<Pattern, PatternError> {
        let _ = &builder;
        builder.build(|src| StrDoc::try_new(src, *self))
      }
    }
    #[cfg(feature=$feature)]
    impl LanguageExt for $name {
      fn get_ts_language(&self) -> TSLanguage {
        parsers::$function()
      }
    }
  };
}
macro_rules! declare_alias {
  ($feature:literal,$name:ident,$aliases:expr) => {
    #[cfg(feature=$feature)]
    impl Alias for $name {
      const ALIAS: &'static [&'static str] = &$aliases;
    }
    #[cfg(feature=$feature)]
    impl fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
      }
    }
    #[cfg(feature=$feature)]
    impl<'de> Deserialize<'de> for $name {
      fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        registry_support::validate_alias(d, Self::ALIAS)?;
        Ok($name)
      }
    }
    #[cfg(feature=$feature)]
    impl From<$name> for SupportLang {
      fn from(_: $name) -> Self {
        Self::$name
      }
    }
  };
}

#[macro_use]
mod registry;
with_languages!(emit_registry);
impl Language for SupportLang {
  fn kind_to_id(&self, kind: &str) -> u16 {
    let _ = &kind;
    with_languages!(emit_dispatch, self, kind_to_id, (kind))
  }
  fn field_to_id(&self, field: &str) -> Option<u16> {
    let _ = &field;
    with_languages!(emit_dispatch, self, field_to_id, (field))
  }
  fn meta_var_char(&self) -> char {
    with_languages!(emit_dispatch, self, meta_var_char, ())
  }
  fn expando_char(&self) -> char {
    with_languages!(emit_dispatch, self, expando_char, ())
  }
  fn extract_meta_var(&self, source: &str) -> Option<MetaVariable> {
    let _ = &source;
    with_languages!(emit_dispatch, self, extract_meta_var, (source))
  }
  fn build_pattern(&self, builder: &PatternBuilder) -> Result<Pattern, PatternError> {
    let _ = &builder;
    with_languages!(emit_dispatch, self, build_pattern, (builder))
  }
  fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str> {
    let _ = &query;
    with_languages!(emit_dispatch, self, pre_process_pattern, (query))
  }
  fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
    from_extension(path.as_ref())
  }
}
impl LanguageExt for SupportLang {
  fn get_ts_language(&self) -> TSLanguage {
    with_languages!(emit_dispatch, self, get_ts_language, ())
  }
  fn injectable_languages(&self) -> Option<&'static [&'static str]> {
    with_languages!(emit_dispatch, self, injectable_languages, ())
  }
  fn extract_injections<L: LanguageExt>(
    &self,
    root: Node<StrDoc<L>>,
  ) -> Vec<(String, Vec<TSRange>)> {
    let _ = &root;
    with_languages!(emit_dispatch, self, extract_injections, (root))
  }
}
#[cfg(test)]
mod test {
  use super::*;
  use ast_grep_core::{Pattern, matcher::MatcherExt};

  pub fn test_match_lang(query: &str, source: &str, lang: impl LanguageExt) {
    let cand = lang.ast_grep(source);
    let pattern = Pattern::new(query, lang);
    assert!(
      pattern.find_node(cand.root()).is_some(),
      "goal: {pattern:?}, candidate: {}",
      cand.root().get_inner_node().to_sexp(),
    );
  }

  pub fn test_non_match_lang(query: &str, source: &str, lang: impl LanguageExt) {
    let cand = lang.ast_grep(source);
    let pattern = Pattern::new(query, lang);
    assert!(
      pattern.find_node(cand.root()).is_none(),
      "goal: {pattern:?}, candidate: {}",
      cand.root().get_inner_node().to_sexp(),
    );
  }

  pub fn test_replace_lang(
    src: &str,
    pattern: &str,
    replacer: &str,
    lang: impl LanguageExt,
  ) -> String {
    let mut source = lang.ast_grep(src);
    assert!(
      source
        .replace(pattern, replacer)
        .expect("should parse successfully")
    );
    source.generate()
  }

  #[cfg(feature = "tree-sitter-javascript")]
  #[test]
  fn test_js_string() {
    test_match_lang("'a'", "'a'", JavaScript);
    test_match_lang("\"\"", "\"\"", JavaScript);
    test_match_lang("''", "''", JavaScript);
  }

  #[cfg(feature = "tree-sitter-rust")]
  #[test]
  fn test_guess_by_extension() {
    let path = Path::new("foo.rs");
    assert_eq!(from_extension(path), Some(SupportLang::Rust));
  }

  #[cfg(feature = "tree-sitter-md")]
  #[test]
  fn test_markdown_extensions() {
    let path = Path::new("README.md");
    assert_eq!(from_extension(path), Some(SupportLang::Markdown));
    let path = Path::new("README.markdown");
    assert_eq!(from_extension(path), Some(SupportLang::Markdown));
  }

  #[cfg(feature = "tree-sitter-zig")]
  #[test]
  fn test_zig_extension_and_alias() {
    let path = Path::new("build.zig");
    assert_eq!(from_extension(path), Some(SupportLang::Zig));
    assert_eq!("zig".parse::<SupportLang>().unwrap(), SupportLang::Zig);
  }

  #[cfg(not(feature = "tree-sitter-kotlin"))]
  #[test]
  fn test_compiled_out_language_is_not_registered() {
    for name in ["kotlin", "kt"] {
      match name.parse::<SupportLang>() {
        Err(SupportLangErr::LanguageNotSupported(rejected)) => assert_eq!(rejected, name),
        Ok(lang) => panic!("compiled-out language resolved to {lang}"),
      }
    }
    for extension in ["kt", "ktm", "kts"] {
      assert_eq!(
        from_extension(Path::new(&format!("example.{extension}"))),
        None
      );
    }
  }

  #[cfg(feature = "builtin-parser")]
  #[test]
  fn test_default_registry_is_unchanged() {
    let names: Vec<_> = SupportLang::all_langs()
      .iter()
      .map(ToString::to_string)
      .collect();
    assert_eq!(
      names,
      [
        "Bash",
        "C",
        "Cpp",
        "CSharp",
        "Css",
        "Dart",
        "Elixir",
        "Go",
        "Haskell",
        "Hcl",
        "Html",
        "Java",
        "JavaScript",
        "Json",
        "Kotlin",
        "Lua",
        "Markdown",
        "Nix",
        "Php",
        "Python",
        "Ruby",
        "Rust",
        "Scala",
        "Solidity",
        "Swift",
        "Tsx",
        "TypeScript",
        "Yaml",
        "Zig"
      ]
    );
    for &lang in SupportLang::all_langs() {
      for name in alias(lang) {
        assert_eq!(name.parse::<SupportLang>().unwrap(), lang);
      }
      let types = lang.file_types();
      for extension in extensions(lang) {
        let path = format!("example.{extension}");
        assert!(types.matched(Path::new(&path), false).is_whitelist());
      }
    }
  }
  #[test]
  fn registry_matches_independent_main_snapshot() {
    type Snapshot = (
      &'static str,
      &'static [&'static str],
      &'static [&'static str],
      char,
      usize,
    );
    let expected: &[Snapshot] = &[
      (
        "Bash",
        &["bash"],
        &[
          "bash", "bats", "cgi", "command", "env", "fcgi", "ksh", "sh", "tmux", "tool", "zsh",
        ],
        '$',
        0,
      ),
      ("C", &["c"], &["c", "h"], '𐀀', 1),
      (
        "Cpp",
        &["cc", "c++", "cpp", "cxx"],
        &["cc", "hpp", "cpp", "c++", "hh", "cxx", "cu", "ino"],
        '𐀀',
        2,
      ),
      ("CSharp", &["cs", "csharp"], &["cs"], 'µ', 3),
      ("Css", &["css"], &["css", "scss"], '_', 4),
      ("Dart", &["dart"], &["dart"], '$', 5),
      ("Elixir", &["ex", "elixir"], &["ex", "exs"], 'µ', 7),
      ("Go", &["go", "golang"], &["go"], 'µ', 6),
      ("Haskell", &["hs", "haskell"], &["hs"], 'µ', 8),
      (
        "Hcl",
        &["hcl"],
        &["hcl", "nomad", "tf", "tfvars", "workflow"],
        'µ',
        9,
      ),
      ("Html", &["html"], &["html", "htm", "xhtml"], 'z', 10),
      ("Java", &["java"], &["java"], '$', 11),
      (
        "JavaScript",
        &["javascript", "js", "jsx"],
        &["cjs", "js", "mjs", "jsx"],
        '$',
        12,
      ),
      ("Json", &["json"], &["json"], '$', 13),
      ("Kotlin", &["kotlin", "kt"], &["kt", "ktm", "kts"], 'µ', 14),
      ("Lua", &["lua"], &["lua"], '$', 15),
      (
        "Markdown",
        &["markdown", "md"],
        &["markdown", "md"],
        '$',
        16,
      ),
      ("Nix", &["nix"], &["nix"], '_', 17),
      ("Php", &["php"], &["php"], 'µ', 18),
      (
        "Python",
        &["py", "python"],
        &["py", "py3", "pyi", "bzl", "bazel"],
        'µ',
        19,
      ),
      ("Ruby", &["rb", "ruby"], &["rb", "rbw", "gemspec"], 'µ', 20),
      ("Rust", &["rs", "rust"], &["rs"], 'µ', 21),
      ("Scala", &["scala"], &["scala", "sc", "sbt"], '$', 22),
      ("Solidity", &["sol", "solidity"], &["sol"], '$', 23),
      ("Swift", &["swift"], &["swift"], 'µ', 24),
      ("Tsx", &["tsx"], &["tsx"], '$', 25),
      (
        "TypeScript",
        &["ts", "typescript"],
        &["ts", "cts", "mts"],
        '$',
        26,
      ),
      ("Yaml", &["yaml", "yml"], &["yaml", "yml"], '$', 27),
      ("Zig", &["zig"], &["zig"], '_', 28),
    ];
    let active: Vec<_> = SupportLang::all_langs()
      .iter()
      .map(ToString::to_string)
      .collect();
    for &(name, aliases, exts, expando, id) in expected {
      if !active.iter().any(|a| a == name) {
        for alias in aliases {
          assert!(alias.parse::<SupportLang>().is_err());
        }
        for ext in exts {
          assert_eq!(from_extension(Path::new(&format!("sample.{ext}"))), None);
        }
        continue;
      }
      let lang: SupportLang = name.parse().unwrap();
      assert_eq!(lang.to_string(), name);
      assert_eq!(lang as usize, id);
      assert_eq!(alias(lang), aliases);
      assert_eq!(extensions(lang), exts);
      assert_eq!(lang.expando_char(), expando);
      assert_eq!(serde_json::to_string(&lang).unwrap(), format!("\"{name}\""));
      for alias in aliases {
        assert_eq!(
          alias.to_ascii_uppercase().parse::<SupportLang>().unwrap(),
          lang
        );
      }
      let code = "x $A $$$";
      if expando == '$' {
        assert!(matches!(lang.pre_process_pattern(code), Cow::Borrowed(_)));
      }
      for ext in exts {
        assert_eq!(
          from_extension(Path::new(&format!("sample.{ext}"))),
          Some(lang)
        );
      }
    }
    let serialized = serde_json::to_string(SupportLang::all_langs()).unwrap();
    let roundtrip: Vec<SupportLang> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(roundtrip, SupportLang::all_langs());
  }
  const _CONST_REGISTRY_CONTRACT: &[SupportLang] = SupportLang::all_langs();
}

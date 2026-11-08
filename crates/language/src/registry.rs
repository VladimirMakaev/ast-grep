// One source of language facts. rustfmt does not format this macro table or its
// emitters, so keep their layout hand-wrapped. Append new languages: table position
// preserves the public enum discriminants even when earlier features are disabled.
// Go/Elixir alone swap their all_langs order to preserve the historical API.
macro_rules! with_languages {
  ($callback:ident $(, $argument:tt)*) => {
    normalize_languages! {$callback, ($($argument),*); 0; []; [
      Bash {
        feature: "tree-sitter-bash", module: bash,
        parser: language_bash => tree_sitter_bash::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["bash"],
        extensions: [
          "bash", "bats", "cgi", "command", "env", "fcgi", "ksh", "sh", "tmux", "tool", "zsh",
        ],
      },
      C {
        feature: "tree-sitter-c", module: none,
        parser: language_c => tree_sitter_c::LANGUAGE,
        preprocessing: Some('𐀀'), implementation: generated,
        aliases: ["c"], extensions: ["c", "h"],
      },
      Cpp {
        feature: "tree-sitter-cpp", module: cpp,
        parser: language_cpp => tree_sitter_cpp::LANGUAGE,
        preprocessing: Some('𐀀'), implementation: generated,
        aliases: ["cc", "c++", "cpp", "cxx"], extensions: ["cc", "hpp", "cpp", "c++", "hh", "cxx", "cu", "ino"],
      },
      CSharp {
        feature: "tree-sitter-c-sharp", module: csharp,
        parser: language_c_sharp => tree_sitter_c_sharp::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["cs", "csharp"], extensions: ["cs"],
      },
      Css {
        feature: "tree-sitter-css", module: css,
        parser: language_css => tree_sitter_css::LANGUAGE,
        preprocessing: Some('_'), implementation: generated,
        aliases: ["css"], extensions: ["css", "scss"],
      },
      Dart {
        feature: "tree-sitter-dart", module: dart,
        parser: language_dart => tree_sitter_dart::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["dart"], extensions: ["dart"],
      },
      Go {
        feature: "tree-sitter-go", module: go,
        parser: language_go => tree_sitter_go::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["go", "golang"], extensions: ["go"],
        order: next,
      },
      Elixir {
        feature: "tree-sitter-elixir", module: elixir,
        parser: language_elixir => tree_sitter_elixir::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["ex", "elixir"], extensions: ["ex", "exs"],
        order: previous,
      },
      Haskell {
        feature: "tree-sitter-haskell", module: haskell,
        parser: language_haskell => tree_sitter_haskell::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["hs", "haskell"], extensions: ["hs"],
      },
      Hcl {
        feature: "tree-sitter-hcl", module: hcl,
        parser: language_hcl => tree_sitter_hcl::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["hcl"], extensions: ["hcl", "nomad", "tf", "tfvars", "workflow"],
      },
      Html {
        feature: "tree-sitter-html", module: html,
        parser: language_html => tree_sitter_html::LANGUAGE,
        preprocessing: None, implementation: external,
        aliases: ["html"], extensions: ["html", "htm", "xhtml"],
      },
      Java {
        feature: "tree-sitter-java", module: none,
        parser: language_java => tree_sitter_java::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["java"], extensions: ["java"],
      },
      JavaScript {
        feature: "tree-sitter-javascript", module: none,
        parser: language_javascript => tree_sitter_javascript::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["javascript", "js", "jsx"], extensions: ["cjs", "js", "mjs", "jsx"],
      },
      Json {
        feature: "tree-sitter-json", module: json,
        parser: language_json => tree_sitter_json::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["json"], extensions: ["json"],
      },
      Kotlin {
        feature: "tree-sitter-kotlin", module: kotlin,
        parser: language_kotlin => tree_sitter_kotlin::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["kotlin", "kt"], extensions: ["kt", "ktm", "kts"],
      },
      Lua {
        feature: "tree-sitter-lua", module: lua,
        parser: language_lua => tree_sitter_lua::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["lua"], extensions: ["lua"],
      },
      Markdown {
        feature: "tree-sitter-md", module: markdown,
        parser: language_markdown => tree_sitter_md::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["markdown", "md"], extensions: ["markdown", "md"],
      },
      Nix {
        feature: "tree-sitter-nix", module: nix,
        parser: language_nix => tree_sitter_nix::LANGUAGE,
        preprocessing: Some('_'), implementation: generated,
        aliases: ["nix"], extensions: ["nix"],
      },
      Php {
        feature: "tree-sitter-php", module: php,
        parser: language_php => tree_sitter_php::LANGUAGE_PHP_ONLY,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["php"], extensions: ["php"],
      },
      Python {
        feature: "tree-sitter-python", module: python,
        parser: language_python => tree_sitter_python::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["py", "python"], extensions: ["py", "py3", "pyi", "bzl", "bazel"],
      },
      Ruby {
        feature: "tree-sitter-ruby", module: ruby,
        parser: language_ruby => tree_sitter_ruby::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["rb", "ruby"], extensions: ["rb", "rbw", "gemspec"],
      },
      Rust {
        feature: "tree-sitter-rust", module: rust,
        parser: language_rust => tree_sitter_rust::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["rs", "rust"], extensions: ["rs"],
      },
      Scala {
        feature: "tree-sitter-scala", module: scala,
        parser: language_scala => tree_sitter_scala::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["scala"], extensions: ["scala", "sc", "sbt"],
      },
      Solidity {
        feature: "tree-sitter-solidity", module: solidity,
        parser: language_solidity => tree_sitter_solidity::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["sol", "solidity"], extensions: ["sol"],
      },
      Swift {
        feature: "tree-sitter-swift", module: swift,
        parser: language_swift => tree_sitter_swift::LANGUAGE,
        preprocessing: Some('µ'), implementation: generated,
        aliases: ["swift"], extensions: ["swift"],
      },
      Tsx {
        feature: "tree-sitter-typescript", module: none,
        parser: language_tsx => tree_sitter_typescript::LANGUAGE_TSX,
        preprocessing: None, implementation: generated,
        aliases: ["tsx"], extensions: ["tsx"],
      },
      TypeScript {
        feature: "tree-sitter-typescript", module: none,
        parser: language_typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        preprocessing: None, implementation: generated,
        aliases: ["ts", "typescript"], extensions: ["ts", "cts", "mts"],
      },
      Yaml {
        feature: "tree-sitter-yaml", module: yaml,
        parser: language_yaml => tree_sitter_yaml::LANGUAGE,
        preprocessing: None, implementation: generated,
        aliases: ["yaml", "yml"], extensions: ["yaml", "yml"],
      },
      Zig {
        feature: "tree-sitter-zig", module: zig,
        parser: language_zig => tree_sitter_zig::LANGUAGE,
        preprocessing: Some('_'), implementation: generated,
        aliases: ["zig"], extensions: ["zig"],
      },
    ]}
  };
}

// Parse the named schema once. Emitters receive stable facets: identity, lookup
// data, root items, and parser items. New record fields belong here; consumers
// that do not use a facet capture it as a single token tree.
macro_rules! normalize_languages {
  ($callback:ident, $arguments:tt; $position:expr; [$($records:tt)*]; []) => {
    $callback! {$arguments; [$($records)*]}
  };
  (
    $callback:ident, $arguments:tt; $position:expr; [$($records:tt)*]; [
      $name:ident {
        feature: $feature:literal, module: $module:ident,
        parser: $function:ident => $parser:path,
        preprocessing: $pre:expr, implementation: $kind:ident,
        aliases: $aliases:expr, extensions: $extensions:expr,
        $(order: $order:ident,)?
      }, $($rest:tt)*
    ]
  ) => {
    normalize_languages! {$callback, $arguments; $position + 1; [
      $($records)*
      $name $feature
      ($position, language_order!($position $(, $order)?))
      ($aliases, $extensions)
      {
        declare_module!($feature, $module);
        declare_marker!($feature, $name, $function, $pre, $kind);
        declare_alias!($feature, $name, $aliases);
      }
      {
        #[cfg(feature = $feature)]
        pub fn $function() -> ast_grep_core::tree_sitter::TSLanguage { $parser.into() }
      }
    ]; [$($rest)*]}
  };
  ($($invalid:tt)*) => {
    compile_error!(concat!(
      "malformed language record: expected Name { feature, module, parser, ",
      "preprocessing, implementation, aliases, extensions, [order] } with trailing commas"
    ));
  };
}

macro_rules! language_order {
  ($position:expr) => {
    $position
  };
  ($position:expr, next) => {
    $position + 1
  };
  ($position:expr, previous) => {
    $position - 1
  };
  ($position:expr, $invalid:ident) => {
    compile_error!("language order override must be next or previous");
  };
}

macro_rules! emit_registry {
  ((); [$(
    $name:ident $feature:literal ($enum_id:expr, $order:expr)
    ($aliases:expr, $extensions:expr) {$($items:tt)*} $parsers:tt
  )*]) => {
    #[cfg(any($(feature = $feature),*))]
    mod registry_support;
    $($($items)*)*
    #[cfg(feature = "tree-sitter-html")]
    pub use html::Html;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash)]
    pub enum SupportLang { $(#[cfg(feature = $feature)] $name = $enum_id,)* }

    impl SupportLang {
      const ALL: &'static [Self] = &Self::ordered([$(#[cfg(feature = $feature)] Self::$name,)*]);
      const fn rank(self) -> usize {
        match self { $(#[cfg(feature = $feature)] Self::$name => $order,)* }
      }
      const fn ordered<const N: usize>(mut languages: [Self; N]) -> [Self; N] {
        let mut i = 0;
        while i < N {
          let mut j = i;
          while j > 0 && languages[j - 1].rank() > languages[j].rank() {
            languages.swap(j - 1, j);
            j -= 1;
          }
          i += 1;
        }
        languages
      }
      pub const fn all_langs() -> &'static [Self] { Self::ALL }
      pub fn file_types(&self) -> Types { file_types(*self) }
    }
    impl fmt::Display for SupportLang {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
    }
    const fn alias(lang: SupportLang) -> &'static [&'static str] {
      match lang { $(#[cfg(feature = $feature)] SupportLang::$name => &$aliases,)* }
    }
    fn extensions(lang: SupportLang) -> &'static [&'static str] {
      match lang { $(#[cfg(feature = $feature)] SupportLang::$name => &$extensions,)* }
    }
    #[cfg(any($(feature = $feature),*))]
    mod parsers;
  };
}

macro_rules! emit_dispatch {
  (($me:expr, $method:ident, $arguments:tt); [$(
    $name:ident $feature:literal $identity:tt $lookup:tt $items:tt $parsers:tt
  )*]) => {
    match *$me { $(#[cfg(feature = $feature)] SupportLang::$name => $name.$method $arguments,)* }
  };
}

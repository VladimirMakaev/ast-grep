macro_rules! with_languages { ($callback:ident $(,$argument:tt)*) => {$callback!{($($argument),*);[
  Bash {
    feature: "tree-sitter-bash", enum_id: 0, order: 0, module: bash,
    parser: language_bash => tree_sitter_bash::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["bash"], extensions: ["bash", "bats", "cgi", "command", "env", "fcgi", "ksh", "sh", "tmux", "tool", "zsh"],
  },
  C {
    feature: "tree-sitter-c", enum_id: 1, order: 1, module: none,
    parser: language_c => tree_sitter_c::LANGUAGE, preprocessing: Some('𐀀'), implementation: generated,
    aliases: ["c"], extensions: ["c", "h"],
  },
  Cpp {
    feature: "tree-sitter-cpp", enum_id: 2, order: 2, module: cpp,
    parser: language_cpp => tree_sitter_cpp::LANGUAGE, preprocessing: Some('𐀀'), implementation: generated,
    aliases: ["cc", "c++", "cpp", "cxx"], extensions: ["cc", "hpp", "cpp", "c++", "hh", "cxx", "cu", "ino"],
  },
  CSharp {
    feature: "tree-sitter-c-sharp", enum_id: 3, order: 3, module: csharp,
    parser: language_c_sharp => tree_sitter_c_sharp::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["cs", "csharp"], extensions: ["cs"],
  },
  Css {
    feature: "tree-sitter-css", enum_id: 4, order: 4, module: css,
    parser: language_css => tree_sitter_css::LANGUAGE, preprocessing: Some('_'), implementation: generated,
    aliases: ["css"], extensions: ["css", "scss"],
  },
  Dart {
    feature: "tree-sitter-dart", enum_id: 5, order: 5, module: dart,
    parser: language_dart => tree_sitter_dart::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["dart"], extensions: ["dart"],
  },
  Go {
    feature: "tree-sitter-go", enum_id: 6, order: 7, module: go,
    parser: language_go => tree_sitter_go::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["go", "golang"], extensions: ["go"],
  },
  Elixir {
    feature: "tree-sitter-elixir", enum_id: 7, order: 6, module: elixir,
    parser: language_elixir => tree_sitter_elixir::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["ex", "elixir"], extensions: ["ex", "exs"],
  },
  Haskell {
    feature: "tree-sitter-haskell", enum_id: 8, order: 8, module: haskell,
    parser: language_haskell => tree_sitter_haskell::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["hs", "haskell"], extensions: ["hs"],
  },
  Hcl {
    feature: "tree-sitter-hcl", enum_id: 9, order: 9, module: hcl,
    parser: language_hcl => tree_sitter_hcl::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["hcl"], extensions: ["hcl", "nomad", "tf", "tfvars", "workflow"],
  },
  Html {
    feature: "tree-sitter-html", enum_id: 10, order: 10, module: html,
    parser: language_html => tree_sitter_html::LANGUAGE, preprocessing: None, implementation: external,
    aliases: ["html"], extensions: ["html", "htm", "xhtml"],
  },
  Java {
    feature: "tree-sitter-java", enum_id: 11, order: 11, module: none,
    parser: language_java => tree_sitter_java::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["java"], extensions: ["java"],
  },
  JavaScript {
    feature: "tree-sitter-javascript", enum_id: 12, order: 12, module: none,
    parser: language_javascript => tree_sitter_javascript::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["javascript", "js", "jsx"], extensions: ["cjs", "js", "mjs", "jsx"],
  },
  Json {
    feature: "tree-sitter-json", enum_id: 13, order: 13, module: json,
    parser: language_json => tree_sitter_json::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["json"], extensions: ["json"],
  },
  Kotlin {
    feature: "tree-sitter-kotlin", enum_id: 14, order: 14, module: kotlin,
    parser: language_kotlin => tree_sitter_kotlin::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["kotlin", "kt"], extensions: ["kt", "ktm", "kts"],
  },
  Lua {
    feature: "tree-sitter-lua", enum_id: 15, order: 15, module: lua,
    parser: language_lua => tree_sitter_lua::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["lua"], extensions: ["lua"],
  },
  Markdown {
    feature: "tree-sitter-md", enum_id: 16, order: 16, module: markdown,
    parser: language_markdown => tree_sitter_md::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["markdown", "md"], extensions: ["markdown", "md"],
  },
  Nix {
    feature: "tree-sitter-nix", enum_id: 17, order: 17, module: nix,
    parser: language_nix => tree_sitter_nix::LANGUAGE, preprocessing: Some('_'), implementation: generated,
    aliases: ["nix"], extensions: ["nix"],
  },
  Php {
    feature: "tree-sitter-php", enum_id: 18, order: 18, module: php,
    parser: language_php => tree_sitter_php::LANGUAGE_PHP_ONLY, preprocessing: Some('µ'), implementation: generated,
    aliases: ["php"], extensions: ["php"],
  },
  Python {
    feature: "tree-sitter-python", enum_id: 19, order: 19, module: python,
    parser: language_python => tree_sitter_python::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["py", "python"], extensions: ["py", "py3", "pyi", "bzl", "bazel"],
  },
  Ruby {
    feature: "tree-sitter-ruby", enum_id: 20, order: 20, module: ruby,
    parser: language_ruby => tree_sitter_ruby::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["rb", "ruby"], extensions: ["rb", "rbw", "gemspec"],
  },
  Rust {
    feature: "tree-sitter-rust", enum_id: 21, order: 21, module: rust,
    parser: language_rust => tree_sitter_rust::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["rs", "rust"], extensions: ["rs"],
  },
  Scala {
    feature: "tree-sitter-scala", enum_id: 22, order: 22, module: scala,
    parser: language_scala => tree_sitter_scala::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["scala"], extensions: ["scala", "sc", "sbt"],
  },
  Solidity {
    feature: "tree-sitter-solidity", enum_id: 23, order: 23, module: solidity,
    parser: language_solidity => tree_sitter_solidity::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["sol", "solidity"], extensions: ["sol"],
  },
  Swift {
    feature: "tree-sitter-swift", enum_id: 24, order: 24, module: swift,
    parser: language_swift => tree_sitter_swift::LANGUAGE, preprocessing: Some('µ'), implementation: generated,
    aliases: ["swift"], extensions: ["swift"],
  },
  Tsx {
    feature: "tree-sitter-typescript", enum_id: 25, order: 25, module: none,
    parser: language_tsx => tree_sitter_typescript::LANGUAGE_TSX, preprocessing: None, implementation: generated,
    aliases: ["tsx"], extensions: ["tsx"],
  },
  TypeScript {
    feature: "tree-sitter-typescript", enum_id: 26, order: 26, module: none,
    parser: language_typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT, preprocessing: None, implementation: generated,
    aliases: ["ts", "typescript"], extensions: ["ts", "cts", "mts"],
  },
  Yaml {
    feature: "tree-sitter-yaml", enum_id: 27, order: 27, module: yaml,
    parser: language_yaml => tree_sitter_yaml::LANGUAGE, preprocessing: None, implementation: generated,
    aliases: ["yaml", "yml"], extensions: ["yaml", "yml"],
  },
  Zig {
    feature: "tree-sitter-zig", enum_id: 28, order: 28, module: zig,
    parser: language_zig => tree_sitter_zig::LANGUAGE, preprocessing: Some('_'), implementation: generated,
    aliases: ["zig"], extensions: ["zig"],
  },
]}};}
macro_rules! emit_registry { (();[$($name:ident {feature:$feature:literal,enum_id:$enum_id:literal,order:$order:literal,module:$module:ident,parser:$function:ident => $parser:path,preprocessing:$pre:expr,implementation:$kind:ident,aliases:$aliases:expr,extensions:$extensions:expr,}),* $(,)?])=>{
#[cfg(any($(feature=$feature),*))] mod registry_support;
$(declare_module!($feature,$module);declare_marker!($feature,$name,$function,$pre,$kind);declare_alias!($feature,$name,$aliases);)*
#[cfg(feature="tree-sitter-html")] pub use html::Html;
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Hash)] pub enum SupportLang {$(#[cfg(feature=$feature)] $name = $enum_id,)*}
impl SupportLang {
 const ALL: &'static[Self] = &Self::ordered([$(#[cfg(feature=$feature)] Self::$name,)*]);
 const fn rank(self)->usize {match self {$(#[cfg(feature=$feature)] Self::$name=>$order,)*}}
 const fn ordered<const N:usize>(mut a:[Self;N])->[Self;N] {let mut i=0;while i<N {let mut j=i;while j>0 && a[j-1].rank()>a[j].rank(){let t=a[j];a[j]=a[j-1];a[j-1]=t;j-=1;}i+=1;}a}
 pub const fn all_langs()->&'static[Self]{Self::ALL}
 pub fn file_types(&self)->Types{file_types(*self)}
}
impl fmt::Display for SupportLang {fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{self:?}")}}
const fn alias(lang:SupportLang)->&'static[&'static str]{match lang {$(#[cfg(feature=$feature)] SupportLang::$name=>&$aliases,)*}}
fn extensions(lang:SupportLang)->&'static[&'static str]{match lang {$(#[cfg(feature=$feature)] SupportLang::$name=>&$extensions,)*}}
#[cfg(any($(feature=$feature),*))] mod parsers;};}
macro_rules! emit_dispatch { (($me:expr,$method:ident, $arguments:tt);[$($name:ident {feature:$feature:literal,enum_id:$enum_id:literal,order:$order:literal,module:$module:ident,parser:$function:ident => $parser:path,preprocessing:$pre:expr,implementation:$kind:ident,aliases:$aliases:expr,extensions:$extensions:expr,}),* $(,)?])=>{match *$me {$(#[cfg(feature=$feature)] SupportLang::$name=>$name.$method $arguments,)*}};}

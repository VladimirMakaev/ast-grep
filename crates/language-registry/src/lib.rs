use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprArray, ExprPath, ExprTuple, Ident, LitStr};

struct Entry {
  feature: LitStr,
  name: Ident,
  enum_id: Expr,
  order: Expr,
  module: Ident,
  function: Ident,
  parser: Expr,
  pre: Expr,
  kind: Ident,
  aliases: Expr,
  extensions: Expr,
}
fn identifier(expr: &Expr) -> syn::Result<Ident> {
  if let Expr::Path(ExprPath { path, .. }) = expr
    && let Some(id) = path.get_ident()
  {
    return Ok(id.clone());
  }
  Err(syn::Error::new_spanned(
    expr,
    "expected a single identifier",
  ))
}
fn entry(expr: Expr) -> syn::Result<Entry> {
  let Expr::Tuple(ExprTuple { elems, .. }) = expr else {
    return Err(syn::Error::new_spanned(expr, "expected a registry tuple"));
  };
  let fields: Vec<_> = elems.into_iter().collect();
  if fields.len() != 11 {
    return Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "registry tuple needs 11 fields",
    ));
  }
  let Expr::Lit(value) = &fields[0] else {
    return Err(syn::Error::new_spanned(
      &fields[0],
      "feature must be a string",
    ));
  };
  let syn::Lit::Str(feature) = &value.lit else {
    return Err(syn::Error::new_spanned(value, "feature must be a string"));
  };
  Ok(Entry {
    feature: feature.clone(),
    name: identifier(&fields[1])?,
    enum_id: fields[2].clone(),
    order: fields[3].clone(),
    module: identifier(&fields[4])?,
    function: identifier(&fields[5])?,
    parser: fields[6].clone(),
    pre: fields[7].clone(),
    kind: identifier(&fields[8])?,
    aliases: fields[9].clone(),
    extensions: fields[10].clone(),
  })
}
#[proc_macro]
pub fn register_languages(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as ExprArray);
  match input
    .elems
    .into_iter()
    .map(entry)
    .collect::<syn::Result<Vec<_>>>()
  {
    Ok(entries) => generate(entries).into(),
    Err(error) => error.into_compile_error().into(),
  }
}
fn generate(entries: Vec<Entry>) -> proc_macro2::TokenStream {
  let features: Vec<_> = entries.iter().map(|e| &e.feature).collect();
  let modules = entries.iter().map(|e| {
    let f = &e.feature;
    let m = &e.module;
    quote!(declare_module!(#f,#m);)
  });
  let markers = entries.iter().map(|e| {
    let Entry {
      feature: f,
      name: n,
      function: fun,
      pre,
      kind,
      aliases,
      ..
    } = e;
    quote!(declare_marker!(#f,#n,#fun,#pre,#kind);declare_alias!(#f,#n,#aliases);)
  });
  let variants = entries.iter().map(|e| {
    let f = &e.feature;
    let n = &e.name;
    let id = &e.enum_id;
    quote!(#[cfg(feature=#f)]#n=#id,)
  });
  let all = entries.iter().map(|e| {
    let f = &e.feature;
    let n = &e.name;
    quote!(#[cfg(feature=#f)]Self::#n,)
  });
  let ranks = entries.iter().map(|e| {
    let f = &e.feature;
    let n = &e.name;
    let rank = &e.order;
    quote!(#[cfg(feature=#f)]Self::#n=>#rank,)
  });
  let aliases = entries.iter().map(|e| {
    let f = &e.feature;
    let n = &e.name;
    let a = &e.aliases;
    quote!(#[cfg(feature=#f)]SupportLang::#n=>&#a,)
  });
  let extensions = entries.iter().map(|e| {
    let f = &e.feature;
    let n = &e.name;
    let a = &e.extensions;
    quote!(#[cfg(feature=#f)]SupportLang::#n=>&#a,)
  });
  let parsers = entries.iter().map(|e| {
    let f = &e.feature;
    let fun = &e.function;
    let p = &e.parser;
    quote!(#[cfg(feature=#f)]pub fn #fun()->TSLanguage{#p.into()})
  });
  let dispatch = |method: &str, args: proc_macro2::TokenStream| {
    let method = Ident::new(method, proc_macro2::Span::call_site());
    let arms = entries.iter().map(|e| {
      let f = &e.feature;
      let n = &e.name;
      quote!(#[cfg(feature=#f)]Self::#n=>#n.#method(#args),)
    });
    quote!(match *self {#(#arms)*})
  };
  let kind = dispatch("kind_to_id", quote!(kind));
  let field = dispatch("field_to_id", quote!(field));
  let meta = dispatch("meta_var_char", quote!());
  let expando = dispatch("expando_char", quote!());
  let extract = dispatch("extract_meta_var", quote!(source));
  let pattern = dispatch("build_pattern", quote!(builder));
  let preprocess = dispatch("pre_process_pattern", quote!(query));
  let language = dispatch("get_ts_language", quote!());
  let injectable = dispatch("injectable_languages", quote!());
  let injections = dispatch("extract_injections", quote!(root));
  quote! {
    #[cfg(any(#(feature=#features),*))] mod registry_support;
    #(#modules)* #(#markers)*
    #[cfg(feature="tree-sitter-html")] pub use html::Html;
    #[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Hash)] pub enum SupportLang {#(#variants)*}
    impl SupportLang {
      const ALL: &'static[Self] = &Self::ordered([#(#all)*]);
      const fn rank(self)->usize {match self {#(#ranks)*}}
      const fn ordered<const N:usize>(mut a:[Self;N])->[Self;N] {let mut i=0;while i<N {let mut j=i;while j>0 && a[j-1].rank()>a[j].rank(){let t=a[j];a[j]=a[j-1];a[j-1]=t;j-=1;}i+=1;}a}
      pub const fn all_langs()->&'static[Self]{Self::ALL}
      pub fn file_types(&self)->Types{file_types(*self)}
    }
    impl fmt::Display for SupportLang {fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{self:?}")}}
    const fn alias(lang:SupportLang)->&'static[&'static str]{match lang {#(#aliases)*}}
    fn extensions(lang:SupportLang)->&'static[&'static str]{match lang {#(#extensions)*}}
    impl Language for SupportLang {
      fn kind_to_id(&self,kind:&str)->u16{ let _ = &kind;#kind}
      fn field_to_id(&self,field:&str)->Option<u16>{ let _ = &field;#field}
      fn meta_var_char(&self)->char{#meta}
      fn expando_char(&self)->char{#expando}
      fn extract_meta_var(&self,source:&str)->Option<MetaVariable>{ let _ = &source;#extract}
      fn build_pattern(&self,builder:&PatternBuilder)->Result<Pattern,PatternError>{ let _ = &builder;#pattern}
      fn pre_process_pattern<'q>(&self,query:&'q str)->Cow<'q,str>{ let _ = &query;#preprocess}
      fn from_path<P:AsRef<Path>>(path:P)->Option<Self>{from_extension(path.as_ref())}
    }
    impl LanguageExt for SupportLang {
      fn get_ts_language(&self)->TSLanguage{#language}
      fn injectable_languages(&self)->Option<&'static[&'static str]>{#injectable}
      fn extract_injections<L:LanguageExt>(&self,root:Node<StrDoc<L>>)->Vec<(String,Vec<TSRange>)>{ let _ = &root;#injections}
    }
    #[cfg(any(#(feature=#features),*))] mod parsers {use ast_grep_core::tree_sitter::TSLanguage;#(#parsers)*}
  }
}

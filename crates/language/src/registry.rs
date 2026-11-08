macro_rules! register_languages { ([$(($feature:literal,$name:ident,$enum_id:literal,$order:literal,$module:ident,$function:ident,$parser:path,$pre:expr,$kind:ident,$aliases:expr,$extensions:expr)),* $(,)?]) => {
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
impl Language for SupportLang {
fn kind_to_id(&self,kind:&str)->u16{ let _ = &kind;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.kind_to_id(kind),)*}}
fn field_to_id(&self,field:&str)->Option<u16>{ let _ = &field;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.field_to_id(field),)*}}
fn meta_var_char(&self)->char{match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.meta_var_char(),)*}}
fn expando_char(&self)->char{match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.expando_char(),)*}}
fn extract_meta_var(&self,source:&str)->Option<MetaVariable>{ let _ = &source;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.extract_meta_var(source),)*}}
fn build_pattern(&self,builder:&PatternBuilder)->Result<Pattern,PatternError>{ let _ = &builder;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.build_pattern(builder),)*}}
fn pre_process_pattern<'q>(&self,query:&'q str)->Cow<'q,str>{ let _ = &query;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.pre_process_pattern(query),)*}}
fn from_path<P:AsRef<Path>>(path:P)->Option<Self>{from_extension(path.as_ref())}
}
impl LanguageExt for SupportLang{
fn get_ts_language(&self)->TSLanguage{match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.get_ts_language(),)*}}
fn injectable_languages(&self)->Option<&'static[&'static str]>{match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.injectable_languages(),)*}}
fn extract_injections<L:LanguageExt>(&self,root:Node<StrDoc<L>>)->Vec<(String,Vec<TSRange>)>{ let _ = &root;match *self {$(#[cfg(feature=$feature)] Self::$name=>$name.extract_injections(root),)*}}
}
#[cfg(any($(feature=$feature),*))] mod parsers {use ast_grep_core::tree_sitter::TSLanguage;$(#[cfg(feature=$feature)] pub fn $function()->TSLanguage{$parser.into()})*}};}

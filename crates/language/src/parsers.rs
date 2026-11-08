use ast_grep_core::tree_sitter::TSLanguage;
macro_rules! emit_parsers { (();[$($name:ident {feature:$feature:literal,enum_id:$enum_id:literal,order:$order:literal,module:$module:ident,parser:$function:ident => $parser:path,preprocessing:$pre:expr,implementation:$kind:ident,aliases:$aliases:expr,extensions:$extensions:expr,}),* $(,)?])=>{$(#[cfg(feature=$feature)]pub fn $function()->TSLanguage{$parser.into()})*};}
with_languages!(emit_parsers);

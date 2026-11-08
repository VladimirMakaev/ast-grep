macro_rules! emit_parsers {
  ((); [$(
    $name:ident $feature:literal $identity:tt $lookup:tt $items:tt {$($parsers:tt)*}
  )*]) => { $($($parsers)*)* };
}
with_languages!(emit_parsers);

//! Exercise the actual private macro schema without a new test dependency.
use std::process::Command;

fn compile_records(records: &str, emit_items: bool) -> std::process::Output {
  let temporary = std::env::temp_dir().join(format!(
    "ast-grep-registry-diagnostic-{}-{}",
    std::process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_nanos()
  ));
  std::fs::create_dir(&temporary).unwrap();
  let source = temporary.join("rows.rs");
  let registry = concat!(env!("CARGO_MANIFEST_DIR"), "/src/registry.rs");
  let types = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/registry_types.rs"
  );
  let consumer = if emit_items {
    r#"macro_rules! unused {
      ((); [$( $name:ident $feature:literal $id:tt $lookup:tt {$($items:tt)*} $parsers:tt )*])
        => { $($($items)*)* };
    }"#
  } else {
    "macro_rules! unused { ($($tokens:tt)*) => {}; }"
  };
  std::fs::write(
    &source,
    format!(
      r#"#[macro_use]
#[path = {registry:?}]
mod registry;
#[path = {types:?}]
mod types;
use types::*;
use std::{{borrow::Cow, fmt}};
{consumer}
normalize_languages!(unused, (); [{records}]);
"#
    ),
  )
  .unwrap();
  let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
    .args([
      "--edition=2024",
      "--crate-type=lib",
      "--emit=metadata",
      "--error-format=json",
      "--cfg=feature=\"enabled\"",
      "--out-dir",
    ])
    .arg(&temporary)
    .arg(&source)
    .output()
    .unwrap();
  std::fs::remove_dir_all(&temporary).unwrap();
  output
}

#[test]
fn malformed_record_explains_the_schema() {
  let output = compile_records(r#"Broken { feature: "missing-fields" },"#, false);
  let stderr = String::from_utf8(output.stderr).unwrap();
  assert!(
    !output.status.success(),
    "malformed record unexpectedly compiled"
  );
  assert!(
    stderr.contains("malformed language record: expected Name { feature, module, parser"),
    "unexpected diagnostic: {stderr}"
  );
}

#[test]
fn trailing_commas_are_optional() {
  let output = compile_records(
    r#"A {
      feature: "unused", module: none, parser: unused => unused::LANGUAGE,
      preprocessing: None, implementation: generated, aliases: [], extensions: []
    }, B {
      feature: "unused", module: none, parser: unused => unused::LANGUAGE,
      preprocessing: None, implementation: external, aliases: [], extensions: [], enum_id: 7
    }"#,
    false,
  );
  assert!(
    output.status.success(),
    "{}",
    String::from_utf8_lossy(&output.stderr)
  );
}

#[test]
fn invalid_metadata_identifies_the_row() {
  for (tail, fragment, code) in [
    (
      "implementation: generatedd, aliases: [], extensions: []",
      "implementation: generatedd",
      "E0599",
    ),
    (
      "implementation: generated, aliases: [], extensions: [], enum_id: \"wrong\"",
      "enum_id: \"wrong\"",
      "E0308",
    ),
  ] {
    let row = format!(
      "Probe {{ feature: \"enabled\", module: none, parser: unused => unused::LANGUAGE, \
       preprocessing: None, {tail} }}"
    );
    let output = compile_records(&row, true);
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "invalid row compiled");
    let diagnostics: Vec<serde_json::Value> = stderr
      .lines()
      .filter_map(|line| serde_json::from_str(line).ok())
      .collect();
    let errors: Vec<_> = diagnostics
      .iter()
      .filter(|d| d["level"] == "error" && !d["code"].is_null())
      .collect();
    assert_eq!(errors.len(), 1, "expected only the row error: {stderr}");
    assert!(
      errors.iter().any(|d| {
        d["code"]["code"] == code
          && d["spans"].as_array().unwrap().iter().any(|span| {
            span["is_primary"] == true
              && span["text"]
                .as_array()
                .unwrap()
                .iter()
                .any(|line| line["text"].as_str().unwrap().contains(fragment))
          })
      }),
      "diagnostic is not anchored to {fragment}: {stderr}"
    );
  }
}

#[test]
fn external_preprocessing_is_rejected() {
  let output = compile_records(
    r#"Probe { feature: "unused", module: unused, parser: unused => unused::LANGUAGE,
      preprocessing: Some('Q'), implementation: external, aliases: [], extensions: [] }"#,
    true,
  );
  let stderr = String::from_utf8(output.stderr).unwrap();
  assert!(!output.status.success(), "inert preprocessing compiled");
  assert!(
    stderr.contains("external marker owns preprocessing; expected None"),
    "{stderr}"
  );
}

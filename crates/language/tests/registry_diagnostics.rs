//! Exercise the actual private macro schema without a new test dependency.
use std::process::Command;

#[test]
fn malformed_record_explains_the_schema() {
  let temporary = std::env::temp_dir().join(format!(
    "ast-grep-registry-diagnostic-{}-{}",
    std::process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_nanos()
  ));
  std::fs::create_dir(&temporary).unwrap();
  let source = temporary.join("malformed.rs");
  let registry = concat!(env!("CARGO_MANIFEST_DIR"), "/src/registry.rs");
  std::fs::write(
    &source,
    format!(
      r#"#[macro_use]
#[path = {registry:?}]
mod registry;
normalize_languages!(unused, (); 0; []; [Broken {{ feature: "missing-fields" }},]);
"#
    ),
  )
  .unwrap();
  let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
    .args([
      "--edition=2024",
      "--crate-type=lib",
      "--emit=metadata",
      "--out-dir",
    ])
    .arg(&temporary)
    .arg(&source)
    .output()
    .unwrap();
  std::fs::remove_dir_all(&temporary).unwrap();
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

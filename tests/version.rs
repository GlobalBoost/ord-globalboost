use super::*;

#[test]
fn version_flag_prints_version() {
  CommandBuilder::new("--version")
    .stdout_regex("ord-globalboost .*\n")
    .run_and_extract_stdout();
}

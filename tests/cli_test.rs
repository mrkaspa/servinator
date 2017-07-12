include!("test_helper.rs");

use servinator::cli::app_spec;

#[test]
fn config_flag_test() {
    let matches =
        app_spec().get_matches_from(vec!["servinator", "files/", "--config", "demo.json"]);
    let config = matches.value_of("config").map(String::from);
    assert!(config == Some("demo.json".to_string()));

    let matches = app_spec().get_matches_from(vec!["servinator", "files/"]);
    let config = matches.value_of("config").map(String::from);
    assert!(config == None);
}

#[test]
fn dir_param_test() {
    let matches = app_spec().get_matches_from(vec!["servinator", "files/"]);
    let dir = matches
        .value_of("INPUT")
        .expect("Please provide the dir");
    assert!(dir == "files/".to_string());
}

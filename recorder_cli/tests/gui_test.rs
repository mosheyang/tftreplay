// ABOUTME: Basic test to ensure GUI module compiles and initializes properly
// ABOUTME: Does not actually launch the GUI window to avoid blocking tests

#[test]
fn gui_module_compiles() {
    // Just ensure the module is accessible
    use recorder_cli::gui;
    
    // Verify the get_default_output_path function works
    let path = gui::get_default_output_path();
    assert!(path.contains("TFT-"));
    assert!(path.contains(".mp4"));
    assert!(path.contains("Movies/TFT Recorder"));
}

#[test]
fn gui_expand_home_works() {
    use recorder_cli::gui::expand_home;
    
    let expanded = expand_home("~/test");
    assert!(!expanded.to_string_lossy().starts_with("~"));
    
    let no_tilde = expand_home("/absolute/path");
    assert_eq!(no_tilde.to_string_lossy(), "/absolute/path");
}
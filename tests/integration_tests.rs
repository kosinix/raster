extern crate image;

#[test]
fn prepare_test() {
    let _ = std::fs::create_dir_all("./tests/out"); // Make sure test out dir is present. This might be a symptom that we need better error handling in editor::save
}

// TODO: test open and save

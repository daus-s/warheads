pub fn data() -> String {
    let curr_dir = std::env::current_exe().expect("💀 could not get current executable path");

    let mut parent = curr_dir.parent();
    while parent.is_some() && !parent.as_ref().unwrap().file_name().unwrap().eq("warheads") {
        parent = parent.unwrap().parent();
    }

    if parent.is_none() {
        panic!("💀 could not find warheads directory");
    }

    let data_path = parent.unwrap().join("data");

    data_path.to_string_lossy().into_owned()
}

#[test]
fn test_data_path() {
    let path = data();
    let parts = path.split("/").collect::<Vec<_>>();

    assert_eq!(parts[parts.len() - 1], "data");
    assert_eq!(parts[parts.len() - 2], "warheads");
}

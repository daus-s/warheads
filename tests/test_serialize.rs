use std::fs;

#[test]
fn test_player_serialization() {
    let expected = read_from_file();

    assert_eq!(expected, "")
}

fn read_from_file() -> String {
    fs::read_to_string("/tests/data/player.json").expect("failed to read from test file")
}

use warheads::ml::elo_tracker::EloTracker;

#[test]
fn track_elo() {
    let mut tracker = EloTracker::new();

    tracker.process_elo();

    match tracker.save() {
        Ok(_) => println!("✅  Elo data saved successfully"),
        Err(e) => println!("❌  Error saving elo data: {}", e),
    }
}

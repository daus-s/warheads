use std::collections::HashMap;

use crate::format::{bar, space};

pub fn evaluation(name: String, map: &HashMap<String, f64>) -> String {
    let mut s = String::new();
    s.push_str(&bar(80));
    s.push_str(&format!("\n░ {:<77}░\n", name));
    s.push_str(&format!("░{}░\n", space(78)));
    s.push_str(&format!("░{}░\n", bar(78)));
    s.push_str(&format!("░{}░\n", space(78)));

    for (metric, value) in map.iter() {
        s.push_str(&format!("░ {:<77}░\n", format!("{}: {}", metric, value)));
    }
    s.push_str(&format!("░{}░\n", space(78)));
    s.push_str(&bar(80));

    s
}

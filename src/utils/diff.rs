use similar::{ChangeTag, TextDiff};
use colored::*;

pub fn generate_diff(old_content: &str, new_content: &str, filename: &str) -> String {
    let diff = TextDiff::from_lines(old_content, new_content);
    let mut output = String::new();
    
    output.push_str(&format!("--- a/{}\n", filename));
    output.push_str(&format!("+++ b/{}\n", filename));
    
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            output.push_str("@@ ... @@\n");
        }
        
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, color) = match change.tag() {
                    ChangeTag::Delete => ("-", "red"),
                    ChangeTag::Insert => ("+", "green"),
                    ChangeTag::Equal => (" ", "white"),
                };
                
                output.push_str(&format!(
                    "{}{}\n",
                    sign.color(color),
                    change.value().trim_end().color(color)
                ));
            }
        }
    }
    
    output
}

pub fn print_diff(old_content: &str, new_content: &str, filename: &str) {
    let diff_output = generate_diff(old_content, new_content, filename);
    print!("{}", diff_output);
}
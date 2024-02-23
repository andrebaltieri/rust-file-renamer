use std::fs;

fn main() -> std::io::Result<()> {
    let file_list = vec![
        "XXXXX_new-file-name",
    ];

    let current_dir = "PATH_TO_FILES";
    for item in file_list {
        let number = extract_chars(item);
        let old_file = format!("{current_dir}\\Aula_{number}.mp4");
        let new_file = format!("{current_dir}\\{item}.mp4");

        match fs::metadata(&old_file) {
            Ok(_) => fs::rename(old_file, new_file)?,
            Err(_) => println!("File does not exist!"),
        }
    }

    Ok(())
}

fn extract_chars(input: &str) -> String {
    if input.len() >= 6 {
        let fifth_char = input.chars().nth(4).unwrap();
        let sixth_char = input.chars().nth(5).unwrap();
        format!("{}{}", fifth_char, sixth_char)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use crate::extract_chars;

    #[test]
    fn should_extract_chars_from_string() {
        let number = extract_chars("XXXX28_file-name-sample-to-test");
        assert_eq!(number, "28");
    }
}
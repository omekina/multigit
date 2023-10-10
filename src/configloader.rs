use std::fs::read_to_string;
use crate::CONFIG_PATH;


struct MultigitConfig {
    designators: Vec<String>,
    keyfiles: Vec<String>,
}


fn load_file() -> Result<String, ()> {
    let file_contents = match read_to_string(CONFIG_PATH) {
        Ok(contents) => contents,
        Err(_) => return Err(()),
    };
    return Ok(file_contents);
}


/*
Parsing modes
0. Default mode (for getting keys and waiting for section starts)
1. Section mode (for getting section names)

Value modes
2. ssh_key
*/
fn parse_file(file_contents: String) -> Result<MultigitConfig, ()> {
    let mut section = String::new();
    let mut temp = String::new();
    let mut config = MultigitConfig { designators: vec![], keyfiles: vec![] };
    let mut mode: u8 = 0;

    for current_char in file_contents.chars() {
        if mode == 0 && current_char == '[' {
            temp = String::new();
            section = String::new();
            mode = 1;
            continue;
        }
        if mode == 1 && current_char == ']' {
            mode = 0;
            config.designators.push(section);
            section = String::new();
            continue;
        }
        if mode == 0 && current_char == '=' {
            if temp == "ssh_key" { mode = 2; }
            temp = String::new();
            continue;
        }
        if current_char == '\n' && mode == 2 {
            if config.designators.len() == 0 || config.designators.len() <= config.keyfiles.len() {
                return Err(());
            }
            config.keyfiles.push(temp);
            temp = String::new();
            mode = 0;
            continue;
        }
        if current_char == ' ' || current_char == '\n' { continue; }
        if mode == 1 {
            section.push(current_char);
            continue;
        }
        temp.push(current_char);
    }

    if mode == 2 {
        if config.designators.len() == 0 || config.designators.len() <= config.keyfiles.len() {
            return Err(());
        }
        config.keyfiles.push(temp);
    }

    return Ok(config);
}


pub fn run(designator: String) -> Result<String, &'static str> {
    let file_contents = match load_file() {
        Ok(contents) => contents,
        Err(_) => return Err("Error loading ~/.mgitconfig file."),
    };
    let parsed_config = match parse_file(file_contents) {
        Ok(result) => result,
        Err(_) => return Err("Error parsing ~/.mgitconfig file."),
    };

    let mut found: i8 = -1;
    for (index, section) in parsed_config.designators.iter().enumerate() {
        if *section == designator { found = index as i8; }
    }
    if found < 0 { return Err("Space designator not found in ~/.mgitconfig file."); }
    return Ok(parsed_config.keyfiles.get(found as usize).unwrap().to_string());
}

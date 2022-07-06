use std::env;

pub fn get_suffix_from_prefix(prefixes: &[&str]) -> Option<String> {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        let split: Vec<&str> = arg.split(&['-', ' ', '='][..]).filter(|p| !p.is_empty()).collect();

        if split.len() == 2 {
            for prefix in prefixes {
                if split[0] == *prefix {
                    return Some(split[1].to_string());
                }
            }
        }
    }

    return None;
}
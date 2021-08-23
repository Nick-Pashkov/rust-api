use regex::Regex;

pub fn handler_pattern(path: &String) -> String {
    let var_regex = Regex::new(r"^\{.+\}$").unwrap();

    let mut result: Vec<String> = Vec::new();
    let segments: Vec<&str> = path
        .split("/")
        .filter(|x| *x != "")
        .collect();

    for segment in segments {
        // Extract vars
        let vars = match var_regex.captures(segment) {
            Some(var) => {
                let var_name = &var[0][1..var[0].len() - 1];
                format!(r"?P<{}>\w+", var_name)
            },
            None => { segment.to_string() }
        };
        //println!("{:?}", vars);
        result.push(format!("({})", vars));
    }

    format!("^/{}$", result.join("/"))
}
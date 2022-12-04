use std::fs::read_to_string;

pub fn parse_file<F, R>(path: &str, line_fn: F) -> Vec<R>
where F: Fn(&str) -> R {
    let contents = match read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => panic!("Can't open file: {:?}", error)
    };
    let lines = contents.split("\r\n");

    let mut result: Vec<R> = Vec::new();
    for line in lines {
        if line.is_empty() {continue};
        result.push(line_fn(line));
    }

    result
}

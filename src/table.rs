use std::collections;

const BORDER: &str = "+";
const CEIL: &str = "-";
const LINE: &str = "|";

pub fn print_with_headers(rows: &Vec<collections::HashMap<String, String>>, headers: &Vec<String>) {
    let mut sizes: collections::HashMap<String, usize> = collections::HashMap::new();
    rows.iter()
        .for_each(|row: &collections::HashMap<String, String>| {
            for (key, value) in row.iter() {
                let last_value: Option<&usize> = sizes.get(key);
                match last_value {
                    Some(last_value) => {
                        if value.len() > *last_value {
                            sizes.insert(key.to_string(), value.len());
                        }
                    }
                    None => {
                        if key.to_string().len() > value.len() {
                            sizes.insert(key.to_string(), key.to_string().len());
                        } else {
                            sizes.insert(key.to_string(), value.len());
                        }
                    }
                }
            }
        });

    let mut headers_map: collections::HashMap<String, String> = collections::HashMap::new();
    headers.iter().for_each(|header: &String| {
        headers_map.insert(header.to_string(), header.to_string());
    });
    print_separator(&headers, &sizes);
    print_line(&headers, &sizes, &headers_map);
    rows.iter()
        .for_each(|row: &collections::HashMap<String, String>| {
            print_separator(&headers, &sizes);
            print_line(&headers, &sizes, row);
        });
    print_separator(&headers, &sizes);
}

pub fn print(rows: &Vec<collections::HashMap<String, String>>) {
    let headers: Vec<String> = get_headers_from(&rows);
    print_with_headers(&rows, &headers);
}

fn get_headers_from(rows: &Vec<collections::HashMap<String, String>>) -> Vec<String> {
    let mut headers: Vec<String> = Vec::new();
    rows.iter()
        .for_each(|row: &collections::HashMap<String, String>| {
            for (key, _) in row.iter() {
                if !headers.contains(key) {
                    headers.push(key.to_string());
                }
            }
        });
    headers
}

fn print_line(
    headers: &Vec<String>,
    sizes: &collections::HashMap<String, usize>,
    row: &collections::HashMap<String, String>,
) {
    print!("{}", LINE);
    headers.iter().for_each(|header: &String| {
        let size: Option<&usize> = sizes.get(header);
        print!(" {} ", row[header]);
        match size {
            None => {}
            Some(size) => {
                for _ in 0..(*size - row[header].len()) {
                    print!(" ");
                }
            }
        }
        print!("{}", LINE);
    });
    println!("");
}

fn print_separator(headers: &Vec<String>, sizes: &collections::HashMap<String, usize>) {
    print!("{}", BORDER);
    headers.iter().for_each(|header: &String| {
        let size: Option<&usize> = sizes.get(header);
        match size {
            None => {}
            Some(size) => {
                for _ in 0..(*size + 2) {
                    print!("{}", CEIL);
                }
            }
        }
        print!("{}", BORDER);
    });
    println!("");
}

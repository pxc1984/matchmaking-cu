use crate::constants::*;

pub fn get_url(endpoint_path: &str) -> String {
    String::from("http://") + &SERVER_NAME + endpoint_path
}

pub fn get_url_params(endpoint_path: &str, params: Vec<(&str, &str)>) -> String {
    let mut base_url = get_url(endpoint_path);

    let args = params.len();
    if args == 0 {
        return base_url;
    } else {
        base_url += "?";
    }

    for arg in 0..args {
        let pair = params[arg];
        base_url += pair.0;
        base_url += "=";
        base_url += pair.1;
        if arg != args - 1 {
            base_url += "&";
        }
    }

    base_url
}
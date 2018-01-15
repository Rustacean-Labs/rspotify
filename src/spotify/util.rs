use rand::{self, Rng};
use chrono::prelude::*;
use webbrowser;

use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use super::oauth2::{TokenInfo, SpotifyOAuth};
pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + elapsed as i64
}

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(length).collect()
}

/// convert map to query_string, for example:
/// convert
/// `{"redirect_uri":"my_uri",
///  "state":"my-state"
///  "scope":"test-scope"}`
/// to
/// `redirect_uri=my_uri&state=my-state&scope=test-scope`
/// Since hashmap is not sorted, so the order of key-value-pairs
/// may differ from times
pub fn convert_map_to_string(map: &HashMap<&str, &str>) -> String {
    let mut string: String = String::new();
    for (key, &value) in map.iter() {
        string.push_str(key);
        string.push_str("=");
        string.push_str(value);
        string.push_str("&");
    }
    string
}

/// convert query string to map, for example:
/// convert
/// `redirect_uri=my_uri&state=my-state&scope=test-scope`
/// to
/// `{"redirect_uri":"my_uri",
///  "state":"my-state"
///  "scope":"test-scope"}`
pub fn convert_str_to_map(query_str: &mut str) -> HashMap<&str, &str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let tokens: Vec<&str> = query_str
        .split("&")
        .filter(|token| !token.is_empty())
        .collect();
    for token in tokens {
        // match token {
        //     &Some(key_value_pair) => {
        let vec: Vec<&str> = token.split("=").collect();
        map.insert(vec[0], vec[1]);
        // }
        // &None => println!("Nothing here"),
        // }
    }
    map
}
// pub fn prompt_for_user_token(client_id: &str,
//                              client_secret: &str,
//                              redirect_uri: &str,
//                              cache_path: PathBuf)
//                              -> TokenInfo {
//     let oauth = SpotifyOAuth::default()
//         .client_id(client_id)
//         .client_secret(client_secret)
//         .redirect_uri(redirect_uri)
//         .cache_path(cache_path)
//         .build();

// }
// pub fn get_token(spotify_oauth: &SpotifyOAuth) -> TokenInfo {
//     match spotify_oauth.get_cached_token() {
//         Some(token_info) => token_info,
//         None => {
//             let auth_url = spotify_oauth.get_authorize_url(None, None);
//             match webbrowser::open(&auth_url) {
//                 Ok(_) => println!("Opened {} in your browser", auth_url),
//                 Err(why) => {
//                     println!("Error:[{:?}],Please naviage here: {}",
//                              why.description(),
//                              auth_url)
//                 }
//             }

//             let mut input = String::new();
//             match io::stdin().read_line(&mut input) {
//                 Ok(n) => {
//                     println!("{} bytes read", n);
//                     println!("{}", input);
//                 }
//                 Err(error) => println!("error: {}", error),
//             }
//         }
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_covert_str_to_map() {
        let mut query_url = String::from("redirect_uri=my_uri&state=my-state&scope=test-scope&");
        let parameters = convert_str_to_map(&mut query_url);
        match parameters.get("redirect_uri") {
            Some(redirect_uri) => {
                assert_eq!(redirect_uri, &"my_uri");
                println!("{:?}", redirect_uri);
            }
            None => panic!("failed"),
        }
    }
    #[test]
    fn test_convert_map_to_string() {
        let mut map = HashMap::new();
        map.insert("redirect_uri", "my_uri");
        map.insert("state", "my-state");
        map.insert("scope", "test-scope");
        let result = convert_map_to_string(&map);
        // hashmap is not sorted, so the order of key-value-pairs will not
        // follow the insert order 
        assert!(result.contains("redirect_uri=my_uri&"));
        assert!(result.contains("state=my-state&"));
        assert!(result.contains("scope=test-scope&"));
    }
}

use regex::{Regex};
use crate::{Game};

pub fn fuz(search_query:String, searched_games:Vec<Game>)->Vec<Game>{
    let mut constucted_pattern = String::with_capacity(search_query.len() * 4 + 2);
    for ch in search_query.to_lowercase().chars(){
        constucted_pattern.push(ch);
        constucted_pattern.push_str(".{0,3}"); 
    } 
    let pattern = format!("({})", constucted_pattern); 
    let re = Regex::new(&pattern).unwrap();


    let mut results: Vec<Game> = Vec::new();

    for game in searched_games {
        let home = &game.home.to_lowercase();
        let away = &game.away.to_lowercase();
        if re.is_match(home) || re.is_match(away){
            results.push(game);
        }
    }
    return results;
}


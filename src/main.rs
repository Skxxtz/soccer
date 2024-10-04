use chrono::{DateTime, Datelike, Timelike, Utc};
use chrono_tz::{Europe::Berlin, Tz};
use core::f32;
use prettytable::{cell, format, row, Table};
use reqwest::Error;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use std::env;
mod fuzzy;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Game {
    home: String,
    away: String,
    score_home: usize,
    score_away: usize,
    timestamp: DateTime<Tz>,
    status: String,
    link: String,
}
struct Team {
    standing: String,
    name: String,
    short: String,
    abbrev: String,
    games: String,
    wins: String,
    draws: String,
    losses: String,
    goals: String,
    goal_dif: String,
    points: String,
}

#[derive(Debug)]
pub struct LineUp {
    team: String,
    players: Vec<Player>,
}
impl LineUp {
    fn new() -> Self {
        LineUp {
            team: String::new(),
            players: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Player {
    x_pos: f32,
    y_pos: f32,
    name: String,
    number: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> Result<(), Error> {
    const LINKS:[&'static str;2] = [
        "https://www.sportschau.de/live-und-ergebnisse/fussball/deutschland-bundesliga/spiele-und-ergebnisse",
        "https://www.sportschau.de/live-und-ergebnisse/fussball/uefa-champions-league/spiele-und-ergebnisse",
    ];
    let mut args: Vec<String> = env::args().collect();
    let mut competition: usize = 0;
    if args.len() > 1 {
        match args[1].as_str() {
            "-c" => {
                competition = 1;
                args = args[1..].to_vec();
            }
            _ => {}
        }
        if args.len() > 1 {
            match args[1].as_str() {
                "standings" => {
                    let standings = gather_standings(&LINKS[competition]).await.unwrap();
                    print_standings(standings);
                }
                "scores" => {
                    let scores = gather_scores(&LINKS[competition]).await.unwrap();
                    print_scores(scores);
                }
                "match" => {
                    if args.len() > 2 {
                        let query = args[2].to_string();
                        let selected_match = get_lineup_link(query, &LINKS[competition]);
                        let stadium = construct_stadium();
                        let selected_match = selected_match.await?;
                        let lineups = get_lineup(selected_match).await?;
                        let _ = populate_stadium(lineups, stadium);
                    }
                }
                "matchday" => {
                    let scores = gather_scores(&LINKS[competition]);
                    let standings = gather_standings(&LINKS[competition]);
                    let scores = scores.await.unwrap();
                    let standings = standings.await.unwrap();
                    print_scores(scores);
                    print_standings(standings);
                }
                "--help" => {
                    help();
                }
                "--version" => {
                    println!("Soccer version: {}", env!("CARGO_PKG_VERSION"));
                }
                _ => {
                    println!("No such command. {}", args[1]);
                    print!(
                        "Available commands:\n→ table\n→ scores\n→ matchday\nDefault: scores\n\n"
                    );
                }
            }
        } else {
            let scores = gather_scores(&LINKS[competition]).await.unwrap();
            print_scores(scores);
        }
    } else {
        let scores = gather_scores(&LINKS[competition]).await.unwrap();
        print_scores(scores);
    }

    Ok(())
}

fn help() {
    println!("Available Commands:\n");
    println!("soccer                    Displays the current score");
    println!("soccer standings          Displays the current standings");
    println!("soccer matchday           Displays the current scores and standings.");
    println!("soccer match [team name]  Displays match for [team name]s match.");
    println!("soccer --version          Displays current version");
    println!("soccer --help             Displays current version");
    println!("");
}
// Score Stuff
async fn gather_scores(link: &str) -> Result<Vec<Game>, Error> {
    let body = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&body);
    let mut games: Vec<Game> = Vec::<Game>::new();

    let sel_match = Selector::parse("li.match").unwrap();
    for element in document.select(&sel_match) {
        let sel_teams = Selector::parse("div.team-name").unwrap();
        let sel_status = Selector::parse("div.match-status").unwrap();
        let sel_score_home = Selector::parse("div.match-result-home").unwrap();
        let sel_score_away = Selector::parse("div.match-result-away").unwrap();
        let sel_minute = Selector::parse("div.current-minute").unwrap();
        let sel_link = Selector::parse("div.match-more").unwrap();

        let teams: Vec<_> = element.select(&sel_teams).collect();
        let (home, away) = (teams[0].inner_html(), teams[1].inner_html());

        let timestamp = if let Some(ts) = element.value().attr("data-datetime") {
            let ts = ts.parse::<DateTime<Utc>>().unwrap();
            ts.with_timezone(&Berlin)
        } else {
            let ts = Utc::now();
            ts.with_timezone(&Berlin)
        };

        let score_home_element: Vec<_> = element.select(&sel_score_home).collect();
        let score_home: usize = score_home_element
            .get(0)
            .and_then(|score_element| score_element.first_child())
            .and_then(ElementRef::wrap)
            .map(|child_element| child_element.inner_html())
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(0);

        let score_away_element: Vec<_> = element.select(&sel_score_away).collect();
        let score_away: usize = score_away_element
            .get(0)
            .and_then(|score_element| score_element.first_child())
            .and_then(ElementRef::wrap)
            .map(|child_element| child_element.inner_html())
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(0);

        let match_status: Vec<_> = element.select(&sel_status).collect();
        let stat = match_status[0].inner_html();
        let status = if stat == "Beendet" {
            String::from("OVER")
        } else if stat == "Live" {
            String::from(format!("LIVE"))
        } else {
            String::from("UPCOMING")
        };

        let mut link: String = String::new();
        if let Some(href) = element
            .select(&sel_link)
            .next()
            .and_then(|el| el.select(&Selector::parse("a").unwrap()).next())
            .and_then(|a| a.value().attr("href"))
        {
            link = href.to_string();
        }

        games.push(Game {
            home,
            away,
            score_home,
            score_away,
            timestamp,
            status,
            link,
        })
    }
    Ok(games)
}
fn print_scores(info: Vec<Game>) {
    let mut table = Table::new();
    let now = Utc::now();
    let now = now.with_timezone(&Berlin);

    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Home", "", "Away", "Time",]);
    for item in info {
        let hour_difference: i32 = item.timestamp.hour() as i32 - now.hour() as i32;
        let date_difference: i32 = item.timestamp.day() as i32 - now.day() as i32;
        let mut date = String::new();

        if date_difference == 0 && hour_difference <= 0 {
            date = format!("{}", item.status);
        } else if date_difference == 0 {
            date = format!(
                "Today, {:2}:{:2}",
                item.timestamp.hour(),
                item.timestamp.minute()
            );
        } else if date_difference == 1 {
            date = format!(
                "Tomorrow, {:2}:{:2}",
                item.timestamp.hour(),
                item.timestamp.minute()
            );
        } else if date_difference > 1 {
            date = format!(
                "{}, {:2}:{:2}",
                item.timestamp.weekday(),
                item.timestamp.hour(),
                item.timestamp.minute()
            );
        }

        table.add_row(row![
            cell!(item.home),
            cell!(format!("{} - {}", item.score_home, item.score_away)),
            cell!(item.away),
            cell!(date)
        ]);
    }
    table.printstd();
}
////////////////////////////////////////////////////////////////////////////////////////////////////

// Standing Stuff
async fn gather_standings(link: &str) -> Result<Vec<Team>, Error> {
    let link: String = construct_url("", link.to_string(), "/tabelle");
    let body = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&body);
    let sel_tr = Selector::parse("tr[class^='hs_team_id-']").unwrap();
    let mut teams: Vec<Team> = Vec::<Team>::new();
    for item in document.select(&sel_tr) {
        let text_content: Vec<String> = item.text().map(|s| s.to_string()).collect();
        if text_content.len() >= 10 {
            let mut iter = text_content.into_iter();
            let team = Team {
                standing: iter.next().unwrap(),
                name: iter.next().unwrap(),
                short: iter.next().unwrap(),
                abbrev: iter.next().unwrap(),
                games: iter.next().unwrap(),
                wins: iter.next().unwrap(),
                draws: iter.next().unwrap(),
                losses: iter.next().unwrap(),
                goals: iter.next().unwrap(),
                goal_dif: iter.next().unwrap(),
                points: iter.next().unwrap(),
            };
            teams.push(team);
        }
    }
    Ok(teams)
}
fn print_standings(standings: Vec<Team>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["#", "Team", "GP", "W", "D", "N", "GO", "GD", "P",]);
    for item in standings {
        table.add_row(row![
            cell!(item.standing),
            cell!(item.name),
            cell!(item.games),
            cell!(item.wins),
            cell!(item.draws),
            cell!(item.losses),
            cell!(item.goals),
            cell!(item.goal_dif),
            cell!(item.points),
        ]);
    }
    table.printstd();
}
////////////////////////////////////////////////////////////////////////////////////////////////////

// Line-Up Stuff
async fn get_lineup_link(query_string: String, comp_link: &str) -> Result<String, Error> {
    let mut matching_games: Vec<Game> = Vec::new();
    let mut link = String::new();
    match gather_scores(comp_link).await {
        Ok(games) => {
            matching_games = fuzzy::fuz(query_string, games);
        }
        Err(e) => return Err(e),
    }
    if let Some(probable) = matching_games.get(0) {
        link = probable.link.clone();
    };
    return Ok(link);
}
async fn get_lineup(link: String) -> Result<Vec<LineUp>, Error> {
    let mut line_ups: Vec<_> = Vec::new();
    let mut home_lineup: LineUp = LineUp::new();
    let mut away_lineup: LineUp = LineUp::new();
    let url = construct_url("https://www.sportschau.de", link, "/taktische-aufstellung");
    let body = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&body);

    let sel_home = Selector::parse("div[class^='hs-starter home']").unwrap();
    let sel_away = Selector::parse("div[class^='hs-starter away']").unwrap();
    let sel_player = Selector::parse("div[class^='tactic'").unwrap();
    let sel_home_name = Selector::parse("div.team-shortname-home").unwrap();
    let sel_away_name = Selector::parse("div.team-shortname-away").unwrap();

    let mut home_name = document.select(&sel_home_name);
    if let Some(teamname) = home_name.next() {
        home_lineup.team = teamname.text().collect();
    }
    let mut away_name = document.select(&sel_away_name);
    if let Some(teamname) = away_name.next() {
        away_lineup.team = teamname.text().collect();
    }

    if let Some(hl_div) = document.select(&sel_home).next() {
        let home_lineup_div: Vec<_> = hl_div.select(&sel_player).collect();
        for element in home_lineup_div {
            let x_pos: f32 = element
                .attr("data-xpos")
                .and_then(|e| e.parse::<f32>().ok())
                .unwrap_or(0.0);
            let y_pos: f32 = element
                .attr("data-ypos")
                .and_then(|e| e.parse::<f32>().ok())
                .unwrap_or(0.0);
            let text_content: Vec<_> = element.text().collect();
            let number_u8: u8 = text_content[0].parse::<u8>().unwrap_or(0);
            let number: String = format!("{:02}", number_u8);
            let name: String = text_content[1].to_string();
            let player = Player {
                x_pos,
                y_pos,
                name,
                number,
            };
            home_lineup.players.push(player);
        }
    } else {
        println!("No div found for the home lineup!")
    };
    if let Some(al_div) = document.select(&sel_away).next() {
        let away_lineup_div: Vec<_> = al_div.select(&sel_player).collect();
        for element in away_lineup_div {
            let mut x_pos: f32 = element
                .attr("data-xpos")
                .and_then(|e| e.parse::<f32>().ok())
                .unwrap_or(0.0);
            let y_pos: f32 = element
                .attr("data-ypos")
                .and_then(|e| e.parse::<f32>().ok())
                .unwrap_or(0.0);
            x_pos = 1.0 - x_pos;
            let text_content: Vec<_> = element.text().collect();
            let number_u8: u8 = text_content[0].parse::<u8>().unwrap_or(0);
            let number: String = format!("{:02}", number_u8);
            let name: String = text_content[1].to_string();
            let player = Player {
                x_pos,
                y_pos,
                name,
                number,
            };
            away_lineup.players.push(player);
        }
    } else {
        println!("No div found for the away lineup!")
    };
    home_lineup.players.sort_by(|p, p2| {
        p.x_pos
            .partial_cmp(&p2.x_pos)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    away_lineup.players.sort_by(|p, p2| {
        p.x_pos
            .partial_cmp(&p2.x_pos)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    home_lineup.players.sort_by(|p, p2| {
        p.y_pos
            .partial_cmp(&p2.y_pos)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    away_lineup.players.sort_by(|p, p2| {
        p.y_pos
            .partial_cmp(&p2.y_pos)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    line_ups.push(home_lineup);
    line_ups.push(away_lineup);

    Ok(line_ups)
}
fn construct_stadium() -> Vec<Vec<String>> {
    let mut field: Vec<Vec<String>> = Vec::new();
    let mid_space = " ".to_string().repeat(33);
    let space_16 = " ".to_string().repeat(10);
    let space_full = " ".to_string().repeat(44);
    let border_h = "─".to_string().repeat(44);

    let top: Vec<String> = format!("┌{}┬{}┐", border_h, border_h)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let mid: Vec<String> = format!("│{}│{}│", space_full, space_full)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let border16: Vec<String> = format!("├──────────┐{}│{}┌──────────┤", mid_space, mid_space)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let box16: Vec<String> = format!("│{}│{}│{}│{}│", space_16, mid_space, mid_space, space_16)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let border16c: Vec<String> = format!("├──────────┘{}│{}└──────────┤", mid_space, mid_space)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let bottom: Vec<String> = format!("└{}┴{}┘", border_h, border_h)
        .split("")
        .filter(|&l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    field.push(top);
    for _ in 0..4 {
        field.push(mid.clone());
    }
    field.push(border16);
    for _ in 0..7 {
        field.push(box16.clone());
    }
    field.push(border16c);
    for _ in 0..4 {
        field.push(mid.clone());
    }
    field.push(bottom);

    field
}
fn populate_stadium(lineups: Vec<LineUp>, mut stadium: Vec<Vec<String>>) {
    let width: f32 = 38.0;
    let height: f32 = 17.0;
    for player in &lineups[0].players {
        let x_pos: usize = (&width * player.y_pos).floor() as usize;
        let y_pos: usize = (&height * player.x_pos).floor() as usize;

        if let Some(row_vec) = stadium.get_mut(y_pos + 1) {
            if x_pos > 1 {
                if let Some(first_char) = player.number.get(0..1) {
                    row_vec[x_pos + 6] = first_char.to_string();
                }
                if let Some(first_char) = player.number.get(1..2) {
                    row_vec[x_pos + 7] = first_char.to_string();
                }
            } else {
                if let Some(first_char) = player.number.get(0..1) {
                    row_vec[x_pos + 1] = first_char.to_string();
                }
                if let Some(first_char) = player.number.get(1..2) {
                    row_vec[x_pos + 2] = first_char.to_string();
                }
            }
        };
    }

    for player in &lineups[1].players {
        let x_pos: usize = (&width * player.y_pos).floor() as usize;
        let y_pos: usize = (&height * (player.x_pos)).floor() as usize;
        if let Some(row_vec) = stadium.get_mut(y_pos + 1) {
            let length = row_vec.len();
            if x_pos > 1 {
                if let Some(first_char) = player.number.get(0..1) {
                    row_vec[&length - (x_pos + 8)] = first_char.to_string();
                }
                if let Some(first_char) = player.number.get(1..2) {
                    row_vec[&length - (x_pos + 7)] = first_char.to_string();
                }
            } else {
                if let Some(first_char) = player.number.get(0..1) {
                    row_vec[&length - (x_pos + 3)] = first_char.to_string();
                }
                if let Some(first_char) = player.number.get(1..2) {
                    row_vec[&length - (x_pos + 2)] = first_char.to_string();
                }
            }
        };
    }

    let home_top_border = top_border(lineups[0].team.chars().count());
    let away_top_border = top_border(lineups[1].team.chars().count());
    let space_between: usize =
        stadium[0].len() - home_top_border.chars().count() - away_top_border.chars().count();

    let top: Vec<String> = format!(
        "{home_top_border}{}{away_top_border}",
        " ".repeat(space_between)
    )
    .split("")
    .filter(|&l| !l.is_empty()) // Filter out empty strings
    .map(|l| l.to_string())
    .collect();
    let mid: Vec<String> = format!(
        "│ {} │{}│ {} │",
        lineups[0].team,
        " ".repeat(space_between),
        lineups[1].team
    )
    .split("")
    .filter(|&l| !l.is_empty()) // Filter out empty strings
    .map(|l| l.to_string())
    .collect();
    stadium.insert(0, mid);
    stadium.insert(0, top);
    if let Some(row_vec) = stadium.get_mut(2) {
        let length = row_vec.len() - 1;
        row_vec[0] = "├".to_string();
        row_vec[home_top_border.chars().count() - 1] = "┴".to_string();
        row_vec[length - away_top_border.chars().count() + 1] = "┴".to_string();
        row_vec[length] = "┤".to_string();
    };

    let mut player_name_table: Vec<String> = Vec::new();
    const PADDING_WIDTH: usize = 5;
    let padding = " ".repeat(PADDING_WIDTH);
    for (player1, player2) in lineups[0].players.iter().zip(lineups[1].players.iter()) {
        let spaces_right = " ".repeat(
            stadium[0].len()
                - player1.name.chars().count()
                - player2.name.chars().count()
                - 8
                - 2 * PADDING_WIDTH,
        );
        let line: String = format!(
            "{}{}  {}{}{}  {}{}",
            padding,
            player1.number,
            player1.name,
            spaces_right,
            player2.name,
            player2.number,
            padding
        );
        player_name_table.push(line);
    }
    for line in stadium {
        println!("{}", line.join(""));
    }
    for line in player_name_table {
        println!("{line}");
    }
}
fn top_border(len: usize) -> String {
    format!("╭{}╮", "─".repeat(len + 2))
}
fn construct_url(base: &str, link: String, segment: &str) -> String {
    let parts: Vec<&str> = link.rsplit("/").collect();
    if parts.is_empty() {
        return link;
    }
    let base_url = &link[..link.len() - parts[0].len() - 1];
    return format!("{base}{base_url}{segment}");
}
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::LineUp;

#[derive(Debug)]
pub struct Stadium {
    field: Vec<Vec<String>>,
}
impl Stadium {
    pub fn new() -> Self {
        let mut field: Vec<Vec<String>> = Vec::with_capacity(19);
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
        Self {
            field
        }
    }
    pub fn populate(&mut self, lineups: Vec<LineUp>) {
        let width: f32 = 38.0;
        let height: f32 = 17.0;
        for player in &lineups[0].players {
            let x_pos: usize = (&width * player.y_pos).floor() as usize;
            let y_pos: usize = (&height * player.x_pos).floor() as usize;

            if let Some(row_vec) = self.field.get_mut(y_pos + 1) {
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
            if let Some(row_vec) = self.field.get_mut(y_pos + 1) {
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

        let home_top_border = Self::top_border(lineups[0].team.chars().count());
        let away_top_border = Self::top_border(lineups[1].team.chars().count());
        let space_between: usize =
            self.field[0].len() - home_top_border.chars().count() - away_top_border.chars().count();

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
        self.field.insert(0, mid);
        self.field.insert(0, top);
        if let Some(row_vec) = self.field.get_mut(2) {
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
                self.field[0].len()
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
        for line in &self.field {
            println!("{}", line.join(""));
        }
        for line in player_name_table {
            println!("{line}");
        }
    }
    fn top_border(len: usize) -> String {
        format!("╭{}╮", "─".repeat(len + 2))
    }
}

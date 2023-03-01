use std::path::PathBuf;

pub struct EditorState {
    rows: Vec<String>,

    x_pos: usize,
    y_pos: usize,

    term_w: u16,
    term_h: u16,

    editing: Option<PathBuf>
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            x_pos: 0,
            y_pos: 0,
            term_w: 0,
            term_h: 0,
            editing: None
        }
    }

    pub fn get_rows(&self) -> &Vec<String> {
        &self.rows
    }

    pub fn get_x(&self) -> u16 {
        self.x_pos as u16
    }

    pub fn get_y(&self) -> u16 {
        self.y_pos as u16
    }

    pub fn add_file_contents(&mut self, contents: String) {
        for line in contents.lines() {
            self.rows.push(line.into());
        }
    }

    pub fn update_editing(&mut self, path: PathBuf) {
        if self.editing.is_none() {
            self.editing = Some(path);
        }
    }

    pub fn update_dimensions(&mut self, new_term_w: u16, new_term_h: u16) {
        self.term_w = new_term_w;
        self.term_h = new_term_h;
    }

    pub fn insert_at_cursor(&mut self, kc: char) {
        if !self.has_content() {
            let new_row = String::from(kc);
            self.rows.push(new_row);
        }
        self.rows[self.y_pos].insert(self.x_pos, kc);
        self.x_pos += 1;
    }

    pub fn move_to_next_line(&mut self) {
        let remaining_string: String = self.rows[self.y_pos][0..self.x_pos].into();
        let removed_string: String = self.rows[self.y_pos][self.x_pos..].into();

        self.rows[self.y_pos].clear();
        self.rows[self.y_pos].push_str(&remaining_string);
        self.rows.insert(self.y_pos + 1, removed_string.clone());

        self.y_pos += 1;
        self.x_pos = removed_string.len();
    }

    pub fn remove_at_cursor(&mut self) {
        if !self.has_content() || self.x_pos == 0 {
            return;
        }

        if self.x_pos == self.rows[self.y_pos].len() {
            self.rows[self.y_pos].pop();
        } else {
            self.rows[self.y_pos].remove(self.x_pos);
        }
        self.x_pos -= 1;
    }

    pub fn move_up(&mut self) {
        if !self.has_content() || self.y_pos == 0 {
            return;
        }

        self.y_pos -= 1;
        self.x_pos = self.rows[self.y_pos].len();
    }

    pub fn move_down(&mut self) {
        if !self.has_content()
            || self.y_pos == (self.term_h - 1) as usize
            || self.y_pos == self.rows.len() - 1
        {
            return;
        }

        self.y_pos += 1;
        self.x_pos = self.rows[self.y_pos].len();
    }

    pub fn move_right(&mut self) {
        let current_string = &self.rows[self.y_pos];
        if !self.has_content() || self.x_pos == current_string.len() {
            return;
        }

        self.x_pos += 1;
    }

    pub fn move_left(&mut self) {
        if !self.has_content() || self.x_pos == 0 {
            return;
        }

        self.x_pos -= 1;
    }

    fn has_content(&self) -> bool {
        if self.rows.len() == 0 {
            return false;
        }

        return true;
    }
}

use tui::style::Color;

use crate::cell_options::CellOptions;
use crate::theme::Theme;
pub struct Cell {
    pub initial: bool,
    pub value: usize,
    pub options: CellOptions,
    pub fg: Color,
    pub bg: Color,
    default_bg: Color,
    default_fg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            initial: false,
            value: 0,
            options: CellOptions::default(),
            fg: Theme::default().white,
            bg: Theme::default().dark_grey,
            default_fg: Theme::default().white,
            default_bg: Theme::default().dark_grey,
        }
    }
}

impl Cell {
    pub fn add_option(&mut self, value: usize) {
        self.options.values[value - 1].valid = true;
    }

    pub fn remove_option(&mut self, value: usize) {
        self.options.values[value - 1].valid = false;
    }

    pub fn toggle_option(&mut self, value: usize) {
        self.options.values[value - 1].valid = !self.options.values[value - 1].valid;
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    pub fn reset_bg(&mut self) {
        self.bg = self.default_bg;
    }
    pub fn reset_fg(&mut self) {
        self.fg = self.default_fg;
    }

    pub fn reset_colors(&mut self) {
        self.reset_bg();
        self.reset_fg();

        let mut y = 0;
        loop {
            self.options.values[y].reset_colors();

            y += 1;
            if y == 9 {
                break;
            }
        }
    }
}

// impl From<&str> for Row {
//     fn from(slice: &str) -> Self {
//         Self {
//             string: String::from(slice),
//             len: slice.graphemes(true).count(),
//         }
//     }
// }

// impl Row {
//     pub fn render(&self, start: usize, end: usize) -> String {
//         let end = cmp::min(end, self.string.len());
//         let start = cmp::min(end, start);
//         let mut result = String::new();

//         #[allow(clippy::integer_arithmetic)]
//         for grapheme in self.string[..]
//             .graphemes(true)
//             .skip(start)
//             .take(end - start)
//         {
//             if grapheme != "\t" {
//                 result.push_str(grapheme);
//             } else {
//                 result.push_str(" ");
//             }
//         }
//         result
//     }

//     pub fn len(&self) -> usize {
//         self.len
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len == 0
//     }

//     pub fn as_bytes(&self) -> &[u8] {
//         self.string.as_bytes()
//     }

//     pub fn insert(&mut self, at: usize, c: char) {
//         if at >= self.len() {
//             self.string.push(c);
//             self.len += 1;
//             return;
//         }

//         let mut result = String::new();
//         let mut length = 0;
//         for (index, grapheme) in self.string.graphemes(true).enumerate() {
//             length += 1;
//             if index == at {
//                 length += 1;
//                 result.push(c);
//             }
//             result.push_str(grapheme);
//         }
//         self.len = length;
//         self.string = result;
//     }

//     pub fn append(&mut self, new: &Self) {
//         self.string = format!("{}{}", self.string, new.string);
//         self.len += new.len;
//     }

//     pub fn split(&mut self, at: usize) -> Self {
//         let mut row = String::new();
//         let mut length = 0;
//         let mut splitted_row = String::new();
//         let mut splitted_length = 0;
//         for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
//             if index < at {
//                 length += 1;
//                 row.push_str(grapheme);
//             } else {
//                 splitted_length += 1;
//                 splitted_row.push_str(grapheme);
//             }
//         }
//         self.string = row;
//         self.len = length;

//         Self {
//             string: splitted_row,
//             len: splitted_length,
//         }
//     }

//     pub fn delete(&mut self, at: usize) {
//         if at >= self.len {
//             return;
//         }

//         let mut result = String::new();
//         let mut length = 0;
//         for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
//             if index != at {
//                 length += 1;
//                 result.push_str(grapheme);
//             }
//         }
//         self.len = length;
//         self.string = result;
//     }

//     pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
//         if at > self.len {
//             return None;
//         }

//         let start = if direction == SearchDirection::Forward {
//             at
//         } else {
//             0
//         };
//         let end = if direction == SearchDirection::Forward {
//             self.len
//         } else {
//             at
//         };

//         #[allow(clippy::integer_arithmetic)]
//         let substring: String = self.string[..]
//             .graphemes(true)
//             .skip(start)
//             .take(end - start)
//             .collect();

//         let matching_byte_index = if direction == SearchDirection::Forward {
//             substring.find(query)
//         } else {
//             substring.rfind(query)
//         };

//         if let Some(matching_byte_index) = matching_byte_index {
//             for (grapheme_index, (byte_index, _)) in
//                 substring[..].grapheme_indices(true).enumerate()
//             {
//                 if matching_byte_index == byte_index {
//                     #[allow(clippy::integer_arithmetic)]
//                     return Some(start + grapheme_index);
//                 }
//             }
//         }
//         None
//     }
// }

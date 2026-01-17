use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
};

use crate::data::fake_data::code_snippets;
use crate::ui::theme::*;

pub struct SourceCodeState {
    snippets: Vec<String>,
    current_snippet: usize,
    scroll_offset: usize,
    tick_counter: u64,
}

impl SourceCodeState {
    pub fn new() -> Self {
        let snippets: Vec<String> = code_snippets()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        Self {
            snippets,
            current_snippet: 0,
            scroll_offset: 0,
            tick_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;

        // Scroll every 15 ticks (~4 times per second at 60fps)
        if self.tick_counter % 15 == 0 {
            let current_lines: Vec<&str> = self.snippets[self.current_snippet].lines().collect();

            self.scroll_offset += 1;

            // Switch to next snippet when done scrolling
            if self.scroll_offset > current_lines.len() + 5 {
                self.scroll_offset = 0;
                self.current_snippet = (self.current_snippet + 1) % self.snippets.len();
            }
        }
    }
}

impl Default for SourceCodeState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_source_code(frame: &mut Frame, state: &SourceCodeState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_DIM))
        .title(" SOURCE ")
        .title_style(Style::default().fg(SYNTAX_FUNCTION).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let code = &state.snippets[state.current_snippet];
    let lines: Vec<&str> = code.lines().collect();

    // Apply syntax highlighting
    let styled_lines: Vec<Line> = lines
        .iter()
        .skip(state.scroll_offset)
        .take(inner.height as usize)
        .enumerate()
        .map(|(idx, line)| {
            let line_num = state.scroll_offset + idx + 1;
            let mut spans = vec![
                Span::styled(
                    format!("{:3} │ ", line_num),
                    Style::default().fg(TEXT_DIM),
                ),
            ];

            // Simple syntax highlighting
            spans.extend(highlight_line(line));

            Line::from(spans)
        })
        .collect();

    let code_widget = Paragraph::new(styled_lines);
    frame.render_widget(code_widget, inner);
}

/// Apply basic syntax highlighting to a line of code
fn highlight_line(line: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current_word = String::new();
    let mut in_string = false;
    let mut string_char = '"';
    let mut in_comment = false;

    let keywords = [
        "fn", "let", "mut", "const", "pub", "use", "mod", "struct", "enum",
        "impl", "trait", "async", "await", "return", "if", "else", "match",
        "for", "while", "loop", "break", "continue", "self", "Self",
        "def", "class", "import", "from", "return", "async", "await",
        "SELECT", "FROM", "WHERE", "JOIN", "ON", "AND", "OR", "ORDER", "BY",
        "void", "int", "char", "HANDLE", "LPVOID", "SIZE_T",
        "for", "in", "do", "done", "if", "then", "fi", "echo",
    ];

    let types = [
        "Result", "Vec", "String", "Option", "bool", "u8", "u16", "u32", "u64",
        "i8", "i16", "i32", "i64", "f32", "f64", "usize", "isize",
    ];

    for ch in line.chars() {
        if in_comment {
            current_word.push(ch);
            continue;
        }

        if in_string {
            current_word.push(ch);
            if ch == string_char {
                spans.push(Span::styled(
                    current_word.clone(),
                    Style::default().fg(SYNTAX_STRING),
                ));
                current_word.clear();
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                if !current_word.is_empty() {
                    spans.push(colorize_word(&current_word, &keywords, &types));
                    current_word.clear();
                }
                string_char = ch;
                in_string = true;
                current_word.push(ch);
            }
            '#' | '/' if current_word.is_empty() || current_word == "/" => {
                if ch == '/' && current_word == "/" {
                    in_comment = true;
                    current_word.push(ch);
                } else if ch == '#' {
                    if !current_word.is_empty() {
                        spans.push(colorize_word(&current_word, &keywords, &types));
                        current_word.clear();
                    }
                    in_comment = true;
                    current_word.push(ch);
                } else {
                    current_word.push(ch);
                }
            }
            ' ' | '(' | ')' | '{' | '}' | '[' | ']' | ',' | ':' | ';' | '.' | '=' | '<' | '>' | '&' | '|' | '+' | '-' | '*' | '!' => {
                if !current_word.is_empty() {
                    spans.push(colorize_word(&current_word, &keywords, &types));
                    current_word.clear();
                }
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(TEXT_PRIMARY),
                ));
            }
            _ => {
                current_word.push(ch);
            }
        }
    }

    // Handle remaining word
    if !current_word.is_empty() {
        if in_comment {
            spans.push(Span::styled(current_word, Style::default().fg(SYNTAX_COMMENT)));
        } else if in_string {
            spans.push(Span::styled(current_word, Style::default().fg(SYNTAX_STRING)));
        } else {
            spans.push(colorize_word(&current_word, &keywords, &types));
        }
    }

    spans
}

fn colorize_word(word: &str, keywords: &[&str], types: &[&str]) -> Span<'static> {
    let word_owned = word.to_string();

    if keywords.contains(&word) {
        Span::styled(word_owned, Style::default().fg(SYNTAX_KEYWORD))
    } else if types.contains(&word) {
        Span::styled(word_owned, Style::default().fg(SYNTAX_TYPE))
    } else if word.chars().all(|c| c.is_numeric() || c == '.') {
        Span::styled(word_owned, Style::default().fg(SYNTAX_NUMBER))
    } else if word.starts_with(|c: char| c.is_uppercase()) {
        Span::styled(word_owned, Style::default().fg(SYNTAX_TYPE))
    } else {
        Span::styled(word_owned, Style::default().fg(TEXT_PRIMARY))
    }
}

use ast::token::Token;
use colored::*;

pub struct FrontendError {
    input_expr: String,
    msg: String,
    start: usize,
    end: usize,
}

impl FrontendError {
    pub fn new(input_expr: &String, msg: String, start: usize, end: usize) -> Self {
        Self {
            input_expr: input_expr.clone(),
            msg,
            start,
            end,
        }
    }

    pub fn from_token(input_expr: &String, msg: &str, token: &Token) -> Self {
        let span = token.span();
        let start = span.start();
        let end = span.end();

        Self {
            input_expr: input_expr.clone(),
            msg: msg.to_string(),
            start,
            end,
        }
    }

    fn split_to_lines(string: String) -> Vec<String> {
        let termsize::Size {
            cols: term_width, ..
        } = termsize::get().unwrap();

        let mut lines = Vec::new();
        let mut line_start = 0;

        for (pos, sym) in string.chars().enumerate() {
            if sym != '\n' && pos - line_start != term_width as usize {
                continue;
            }

            lines.push(string[line_start..pos].to_string());
            line_start = if sym == '\n' { pos + 1 } else { pos };
        }

        lines.push(string[line_start..string.len()].to_string());

        lines
    }

    fn get_expr(&self) -> Vec<String> {
        Self::split_to_lines(self.input_expr.clone())
    }

    fn get_underline(&self) -> Vec<String> {
        let mut underline = String::new();

        for (pos, _) in self.input_expr.chars().enumerate() {
            underline.push(if pos >= self.start && pos < self.end {
                '^'
            } else {
                ' '
            })
        }

        Self::split_to_lines(underline)
    }

    fn get_output_expr(&self) -> Vec<(String, String)> {
        let expr = self.get_expr();
        let underline = self.get_underline();
        let mut output_expr = Vec::new();

        for i in 0..expr.len() {
            output_expr.push((expr[i].clone(), underline[i].clone()));
        }

        output_expr
    }
}

impl std::fmt::Display for FrontendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let output_expr = self.get_output_expr();

        writeln!(f, "{} {}.", "Error:".red().bold(), self.msg.bold())?;
        writeln!(f, "{}", "|".blue().bold())?;

        for (expr, underline) in output_expr {
            writeln!(f, "{} {}", "|".blue().bold(), expr.italic())?;
            writeln!(f, "{} {}", "|".blue().bold(), underline.red().bold())?;
            writeln!(f, "{}", "|".blue().bold())?;
        }

        write!(f, "")
    }
}

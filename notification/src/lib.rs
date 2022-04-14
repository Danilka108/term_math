use colored::*;

#[derive(Debug, Clone)]
enum NotificationKind {
    Warn,
    Err,
}

#[derive(Debug, Clone)]
pub struct Notification {
    input_expr: String,
    msg: String,
    start: usize,
    end: usize,
    kind: NotificationKind,
}

impl Notification {
    pub fn new_error(input_expr: &String, msg: String, start: usize, end: usize) -> Self {
        Self::new(NotificationKind::Err, input_expr, msg, start, end)
    }

    pub fn new_warning(input_expr: &String, msg: String, start: usize, end: usize) -> Self {
        Self::new(NotificationKind::Warn, input_expr, msg, start, end)
    }

    fn new(
        kind: NotificationKind,
        input_expr: &String,
        msg: String,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind,
            input_expr: input_expr.clone(),
            msg,
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

impl std::fmt::Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let output_expr = self.get_output_expr();

        let message_prefix = match self.kind {
            NotificationKind::Err => "Error:".red().bold(),
            NotificationKind::Warn => "Warning:".red().yellow(),
        };

        writeln!(f, "{} {}.", message_prefix, self.msg.bold())?;
        writeln!(f, "{}", "|".blue().bold())?;

        for (expr, underline) in output_expr {
            let underline = match self.kind {
                NotificationKind::Err => underline.red().bold(),
                NotificationKind::Warn => underline.yellow().bold(),
            };

            writeln!(f, "{} {}", "|".blue().bold(), expr.italic())?;
            writeln!(f, "{} {}", "|".blue().bold(), underline)?;
            writeln!(f, "{}", "|".blue().bold())?;
        }

        write!(f, "")
    }
}

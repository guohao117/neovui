use iced::Color;

#[derive(PartialEq, Debug, Clone)]
pub struct
Colors {
    pub foregroud: Option<Color>,
    pub backgroud: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub colors: Colors,
    pub reverse: bool,
    pub italic: bool,
    pub bold: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

impl Style {
    pub fn foregroud(&self, default_colors: &Colors) -> Color {
        if self.reverse {
            self.colors.backgroud.clone()
                .unwrap_or_else(|| default_colors.backgroud.clone().unwrap())
        } else {
            self.colors.foregroud.clone()
                .unwrap_or_else(|| default_colors.foregroud.clone().unwrap())
        }
    }

    pub fn backgroud(&self, default_colors: &Colors) -> Color {
        if self.reverse {
            self.colors.foregroud.clone()
                .unwrap_or_else(|| default_colors.foregroud.clone().unwrap())
        } else {
            self.colors.backgroud.clone()
                .unwrap_or_else(|| default_colors.backgroud.clone().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Gray,
    White,
    Green,
    Orange,
    Purple,
    Yellow,
    Blue,
    Red,
}

impl Color {
    pub fn to_color32(&self) -> egui::Color32 {
        match self {
            Color::Gray => egui::Color32::GRAY,
            Color::White => egui::Color32::WHITE,
            Color::Green => egui::Color32::GREEN,
            Color::Orange => egui::Color32::ORANGE,
            Color::Purple => egui::Color32::PURPLE,
            Color::Yellow => egui::Color32::YELLOW,
            Color::Blue => egui::Color32::BLUE,
            Color::Red => egui::Color32::RED,
        }
    }

}    

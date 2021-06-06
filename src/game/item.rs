//! Item customization
//!

#[derive(Debug, Clone)]
pub enum ItemKind {
    Money(u32),
    Wand,
}


impl ItemKind {
    pub fn description(&self) -> String {
        match self {
            ItemKind::Wand => String::from("a magical wand"),
            ItemKind::Money(x) => format!("{} coins of gold", x)
        }
    }
}

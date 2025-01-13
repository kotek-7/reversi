#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disk {
    W, // White
    B, // Black
    E, // Empty
}

impl Disk {
    pub fn rev(&self) -> Disk {
        match *self {
            Disk::W => Disk::B,
            Disk::B => Disk::W,
            Disk::E => Disk::E,
        }
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Disk::W => "◯",
            Disk::B => "●",
            Disk::E => "·",
        };
        write!(f, "{}", char)
    }
}
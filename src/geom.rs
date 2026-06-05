#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    // Funcion para comprobar si un Vec2 esta dentro de un Rect
    pub fn contains(&self, p: Vec2) -> bool {
        let is_on_x = p.x >= self.x && p.x < (self.x + self.width);
        let is_on_y = p.y >= self.y && p.y < (self.y + self.height);

        is_on_x && is_on_y
    }

    // Funcion para comprobar si dos Rects se intersectan
    pub fn intersects(&self, other: &Rect) -> bool {
        let is_on_x = self.x < (other.x + other.width) && (self.x + self.width) > other.x;
        let is_on_y = self.y < (other.y + other.height) && (self.y + self.height) > other.y;

        is_on_x && is_on_y
    }

    // Funcion para expandir un Rect en todos los lados de manera centrada
    pub fn expand(&self, amount: i32) -> Rect {
        Rect::new(
            self.x - amount,
            self.y - amount,
            self.width + (2 * amount),
            self.height + (2 * amount),
        )
    }

    // Funcion para calcular la interseccion entre dos Rects
    pub fn intersect(&self, other: &Rect) -> Rect {
        // Buscamos cual es la esquina superior izquierda del punto de interseccion
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);

        // Buscamos cual es la esquina inferior derecha del punto de interseccion
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        // Obtenemos el ancho y alto del rectangulo de interseccion
        let width = (x2 - x1).max(0);
        let height = (y2 - y1).max(0);

        Rect::new(x1, y1, width, height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum GreaseMode { Fixed(u16, u16), Random, None }

pub fn get_grease_pairs(mode: GreaseMode) -> (u16, u16) {
    match mode {
        GreaseMode::Fixed(x, y) => (x, y), // e.g., 0x0a0a, 0x1a1a en tests
        GreaseMode::Random => rand_grease_pair(), // Función para generar pares aleatorios
        GreaseMode::None => (0,0), // No GREASE
    }
}

// TODO: Implementar rand_grease_pair criptográficamente segura
fn rand_grease_pair() -> (u16, u16) {
    // Implementación placeholder
    (0x0a0a, 0x1a1a) // Devolver valores fijos por ahora
}

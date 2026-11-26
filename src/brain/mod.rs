//! Núcleo cognitivo do Mimic.

mod feelings;
mod memory;
mod thought;

/// Inicializa os módulos cognitivos disponíveis.
pub(super) fn initialize() {
    println!("Inicializando núcleo cognitivo...");
    feelings::initialize();
    thought::initialize();
    memory::initialize();
}

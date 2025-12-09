struct Circulo {
    raio: f64,
}

// Implementação de métodos para a struct Circulo
impl Circulo {
    // Método Construtor (função associada, não precisa de 'self')
    fn new(raio: f64) -> Circulo {
        Circulo { raio }
    }

    // Método de instância (recebe '&self')
    fn calcular_area(&self) -> f64 {
        std::f64::consts::PI * self.raio * self.raio
    }
}
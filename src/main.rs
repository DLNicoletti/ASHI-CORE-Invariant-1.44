// ASHI-CORE v2.0.2 - Mission Critical Rust Engine
// Registered SIAE n. 2026/00008

pub struct AshiCore {
    pub window_size: usize,
    pub threshold_kc: f64, // Validated at 1.441 in Test 4
}

impl AshiCore {
    pub fn new() -> Self {
        Self {
            window_size: 12,
            threshold_kc: 1.441,
        }
    }

    /// Analizza un buffer di telemetria e restituisce true se rileva una transizione di fase
    pub fn check_stability(&self, buffer: &[f64]) -> bool {
        if buffer.len() < self.window_size { return false; }

        let mean = buffer.iter().sum::<f64>() / buffer.len() as f64;
        let variance = buffer.iter()
            .map(|x| {
                let diff = mean - x;
                diff * diff
            })
            .sum::<f64>() / buffer.len() as f64;

        let std_dev = variance.sqrt();
        let k_parameter = std_dev / mean;

        // Se K supera Kc, viene attivato il flag di instabilità
        k_parameter >= self.threshold_kc
    }
}

fn main() {
    let sensor = AshiCore::new();
    println!("ASHI-CORE Engine Online. Monitoring threshold: {}", sensor.threshold_kc);
}

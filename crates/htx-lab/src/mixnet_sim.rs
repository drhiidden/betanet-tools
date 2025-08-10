/// mixnet_sim: simulador minimal de selección de rutas y latencia

pub struct PathSample {
    pub path_id: usize,
    pub hops: usize,
    pub latency_ms: u64,
}

/// Simula `n` rutas y devuelve una muestra de latencias para cada ruta.
pub fn simulate_paths(n: usize, base_ms: u64) -> Vec<PathSample> {
    let mut out = Vec::new();
    for i in 0..n {
        let hops = 2 + (i % 5);
        let latency = base_ms + (i as u64 * 13 % 100) + (hops as u64 * 5);
        out.push(PathSample { path_id: i, hops, latency_ms: latency });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_sim() {
        let s = simulate_paths(5, 20);
        assert_eq!(s.len(), 5);
    }
}

/// bo-core: SANS-IO core traits and types

pub trait Connection {
    fn id(&self) -> u64;
}

pub trait Stream {
    fn is_open(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyConn(u64);
    impl Connection for DummyConn { fn id(&self) -> u64 { self.0 } }

    #[test]
    fn dummy_conn_id() {
        let c = DummyConn(42);
        assert_eq!(c.id(), 42);
    }
}



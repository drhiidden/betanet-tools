// bo-htx: framing y traits HTX (esqueleto)
use bo_core::Result;
use bo_core::transport::AsyncStream;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Connection {
    pub stream: Box<dyn AsyncStream + Send>,
    pub aead_key: Vec<u8>,
    pub ns: Vec<u8>,
    pub counter: Arc<Mutex<u64>>,
}

impl Connection {
    pub fn new(stream: Box<dyn AsyncStream + Send>, aead_key: Vec<u8>, ns: Vec<u8>) -> Self {
        Connection { stream, aead_key, ns, counter: Arc::new(Mutex::new(0)) }
    }

    pub async fn open_stream(&self) -> Result<u64> {
        // stub: return a new stream id (incrementing)
        let mut c = self.counter.lock().await;
        *c += 1;
        Ok(*c)
    }

    pub async fn accept_stream(&self) -> Result<u64> {
        // stub: accept returns a fixed id for now
        Ok(1)
    }
}



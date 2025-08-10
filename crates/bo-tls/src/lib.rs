// bo-tls: wrapper mínimo para rustls que exporta keying material
use bo_core::prelude::*;

pub async fn export_keying_material_example(_server_name: &str) -> Result<Vec<u8>> {
    // Placeholder: implement using rustls::ClientConnection + Stream
    Ok(vec![])
}



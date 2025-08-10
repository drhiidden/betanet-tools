use serde::{Serialize};
use crate::{HelloTemplate, Extension as InternalExtension}; // Alias for clarity

#[derive(Serialize)]
struct UtlsJson {
    tls_vers_min: u16,
    tls_vers_max: u16,
    cipher_suites: Vec<u16>,
    extensions: Vec<UtlsExt>, // espejo 1:1 de uTLS
    // flags auxiliares (grease_fixed, alpn, etc.)
}

// Representación de las extensiones de uTLS en Go
#[derive(Serialize)]
#[serde(untagged)] // Para serializar el enum directamente sin un tag de variante
pub enum UtlsExt {
    SNIExtension { server_name: String },
    SupportedCurvesExtension { curves: Vec<u16> },
    SupportedPointsExtension { supported_points: Vec<u8> },
    SignatureAlgorithmsExtension { algs: Vec<u16> },
    ALPNExtension { alpn_protocols: Vec<String> },
    KeyShareExtension { key_shares: Vec<UtlsKeyShare> },
    PSKKeyExchangeModesExtension { modes: Vec<u8> },
    SupportedVersionsExtension { versions: Vec<u16> },
    ApplicationSettingsExtension { supported_protocols: Vec<String>, data: Vec<u8> },
    GenericExtension { id: u16, data: Vec<u8> }, // Para ECH outer/grease y otras extensiones desconocidas
    GREASEExtension,
    UtlsPaddingExtension { get_padding_len: String }, // Representado como string para la función Go
}

#[derive(Serialize)]
pub struct UtlsKeyShare {
    group: u16,
    data: Vec<u8>,
}

pub fn export_utls_json(t: &HelloTemplate) -> UtlsJson {
    // mapear InternalExtension::{...} → UtlsExt::{...}
    // preservar orden exacto
    // Convertir a la estructura JSON compatible con uTLS
    let extensions = t.extensions.iter().map(|ext| {
        match ext {
            InternalExtension::ServerName { host } => UtlsExt::SNIExtension { server_name: host.clone() },
            InternalExtension::SupportedVersions { versions } => UtlsExt::SupportedVersionsExtension { versions: versions.clone() },
            InternalExtension::SupportedGroups { groups, grease_slots: _ } => UtlsExt::SupportedCurvesExtension { curves: groups.clone() }, // uTLS usa SupportedCurvesExtension para grupos
            InternalExtension::SignatureAlgorithms { algs } => UtlsExt::SignatureAlgorithmsExtension { algs: algs.clone() },
            InternalExtension::KeyShare { shares } => UtlsExt::KeyShareExtension {
                key_shares: shares.iter().map(|(group, key)| UtlsKeyShare { group: *group, data: key.clone() }).collect()
            },
            InternalExtension::Alpn { protocols } => UtlsExt::ALPNExtension { alpn_protocols: protocols.clone() },
            InternalExtension::Padding { len } => UtlsExt::UtlsPaddingExtension { get_padding_len: format!("func(clientHelloLen int) int {{ return {} }}", len) },
            InternalExtension::EchOuterStub { config_id } => UtlsExt::GenericExtension { id: 0xfe0d, data: config_id.clone() },
            InternalExtension::ApplicationSettings { protocols, data } => UtlsExt::ApplicationSettingsExtension { supported_protocols: protocols.clone(), data: data.clone() },
            InternalExtension::Unknown { typ, bytes } => UtlsExt::GenericExtension { id: *typ, data: bytes.clone() },
        }
    }).collect();

    UtlsJson {
        tls_vers_min: 0x0303, // TLS 1.2
        tls_vers_max: 0x0304, // TLS 1.3
        cipher_suites: t.cipher_suites.clone(),
        extensions,
    }
}

pub fn export_utls_go(t: &HelloTemplate) -> String {
    // genera un snippet Go listo para pegar:
    // var spec = &utls.ClientHelloSpec{ TLSVersMin: ..., TLSVersMax: ..., CipherSuites: []uint16{...}, Extensions: []utls.TLSExtension{ ... } }
    todo!()
}

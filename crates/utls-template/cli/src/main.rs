use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::Parser;
use hello_template::{HelloTemplate, Encoder};
use hello_template::export_utls;
use hello_snapshot::pcap_importer::import_from_pcap;
use hello_snapshot::snapshot_to_ja3;
use serde_json;

#[derive(Parser)]
struct Cmd {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Gen {
        template: String,
        #[arg(long)] out: Option<String>,
        #[arg(long)] emit_pcap: bool,
        #[arg(long)] export_utls: Option<String>,
    },
    Selftest {
        #[arg(long)] pcap: String,
        #[arg(long)] template: String,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cmd::parse();
    match cli.cmd {
        Commands::Gen { template, out, emit_pcap, export_utls } => {
            let mut f = File::open(&template)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            let tpl: HelloTemplate = serde_json::from_str(&s)?;
            let enc = Encoder::encode_client_hello(&tpl, emit_pcap)?;
            let out_path = out.unwrap_or_else(|| "clienthello.bin".to_string());
            std::fs::write(&out_path, &enc.raw_bytes)?;
            println!("Wrote {} ({} bytes)", out_path, enc.raw_bytes.len());
            if let Some(p) = enc.pcap_bytes {
                let pcap_path = Path::new(&out_path).with_extension("pcap");
                std::fs::write(&pcap_path, p)?;
                println!("Wrote pcap {}", pcap_path.display());
            }
            if let Some(go_path) = export_utls {
                let go_snip = export_utls::export_utls_go(&tpl);
                std::fs::write(&go_path, go_snip)?;
                println!("Wrote uTLS Go snippet {}", go_path);
            }
        }
        Commands::Selftest { pcap, template } => {
            println!("Importing pcap: {}", pcap);
            let mut snap = import_from_pcap(&pcap).map_err(|e| format!("import pcap failed: {}", e))?;
            println!("Imported snapshot without ja3: {:?}", snap.raw_client_hello.len());

            // Add JA3 calculation
            snapshot_to_ja3(&mut snap);
            println!("PCAP JA3: {}", snap.ja3.clone().unwrap_or_default());

            let mut f = File::open(&template)?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            let tpl: HelloTemplate = serde_json::from_str(&s)?;
            let enc = Encoder::encode_client_hello(&tpl, false)?;
            // compute ja3 of generated
            let mut generated = snap.clone();
            generated.raw_client_hello = enc.raw_bytes;
            snapshot_to_ja3(&mut generated);
            println!("Generated JA3: {}", generated.ja3.clone().unwrap_or_default());

            if snap.ja3 == generated.ja3 {
                println!("SELFTEST PASS: JA3 match");
            } else {
                println!("SELFTEST FAIL: JA3 differ");
                return Err("selftest failed".into());
            }
        }
    }
    Ok(())
}

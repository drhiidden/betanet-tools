#!/usr/bin/env bash
set -euo pipefail

# pcap_sanitizer.sh
# Basic PCAP sanitizer: extracts TLS ClientHello packets (and tries to extract QUIC Initial packets if available)
# Produces: <out_prefix>.sanitized.pcap and <out_prefix>.metadata.json
# Requirements: tshark (Wireshark). For stronger anonymization (IP/MAC rewrite) install tcprewrite (part of tcpreplay).

usage(){
  cat <<EOF
Usage: $0 -i input.pcap -o out_prefix

This script extracts TLS ClientHello frames (and QUIC Initial heuristically) into a small sanitized pcap
and writes a metadata JSON describing the extraction. It does NOT guarantee removal of all PII —
use tcprewrite to rewrite addresses if needed.

Requires: tshark (https://www.wireshark.org/)
Optional: tcprewrite (for address/macs rewriting)
EOF
  exit 1
}

INPUT=""
OUTPREFIX=""
while getopts ":i:o:" opt; do
  case ${opt} in
    i ) INPUT=$OPTARG ;;
    o ) OUTPREFIX=$OPTARG ;;
    * ) usage ;;
  esac
done

if [[ -z "$INPUT" || -z "$OUTPREFIX" ]]; then
  usage
fi

if ! command -v tshark >/dev/null 2>&1; then
  echo "Error: tshark not found. Please install Wireshark/tshark." >&2
  exit 2
fi

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

CLIENTHELLO_PCAP="$TMPDIR/${OUTPREFIX}.clienthello.pcap"
QUIC_PCAP="$TMPDIR/${OUTPREFIX}.quic_initial.pcap"

echo "[*] Extracting TLS ClientHello packets to $CLIENTHELLO_PCAP"
# Extract TLS ClientHello (handshake type 1). Works for TLS over TCP
tshark -r "$INPUT" -Y "ssl.handshake.type == 1" -w "$CLIENTHELLO_PCAP" 2>/dev/null || true

# Attempt to extract QUIC Initial packets (heuristic): use display filter 'quic' when available
echo "[*] Attempting to extract QUIC Initial packets (heuristic) to $QUIC_PCAP"
if tshark -G fields 2>/dev/null | grep -q "quic"; then
  # Extract UDP packets to/from 443 containing QUIC
  tshark -r "$INPUT" -Y "udp.port == 443 and quic.packet_type == 0" -w "$QUIC_PCAP" 2>/dev/null || true
else
  # Fallback: naive UDP 443 capture
  tshark -r "$INPUT" -Y "udp.port == 443" -w "$QUIC_PCAP" 2>/dev/null || true
fi

# Merge clienthello + quic initial into a sanitized pcap (keep order)
SANITIZED_PCAP="${OUTPREFIX}.sanitized.pcap"

# If both files have content, merge via mergecap if available, else concat via editcap (preserve simple merge)
if command -v mergecap >/dev/null 2>&1; then
  echo "[*] Merging extracted pcaps into $SANITIZED_PCAP"
  mergecap -w "$SANITIZED_PCAP" "$CLIENTHELLO_PCAP" "$QUIC_PCAP" >/dev/null 2>&1 || true
else
  # fallback: prefer clienthello, then append quic by creating new pcap
  if [[ -s "$CLIENTHELLO_PCAP" ]]; then
    cp "$CLIENTHELLO_PCAP" "$SANITIZED_PCAP"
    if [[ -s "$QUIC_PCAP" ]]; then
      echo "Warning: mergecap not found, QUIC packets won't be appended. Install mergecap (part of Wireshark) for merging." >&2
    fi
  elif [[ -s "$QUIC_PCAP" ]]; then
    cp "$QUIC_PCAP" "$SANITIZED_PCAP"
  else
    echo "No TLS ClientHello or QUIC Initial packets found in input. Exiting." >&2
    exit 3
  fi
fi

# Metadata
META_JSON="${OUTPREFIX}.metadata.json"
CLIENT_CNT=0
QUIC_CNT=0
if [[ -s "$CLIENTHELLO_PCAP" ]]; then
  CLIENT_CNT=$(tshark -r "$CLIENTHELLO_PCAP" -q -z io,phs 2>/dev/null | wc -l || echo 1)
fi
if [[ -s "$QUIC_PCAP" ]]; then
  QUIC_CNT=$(tshark -r "$QUIC_PCAP" -q -z io,phs 2>/dev/null | wc -l || echo 1)
fi

cat > "$META_JSON" <<JSON
{
  "input": "${INPUT}",
  "sanitized_pcap": "${SANITIZED_PCAP}",
  "clienthello_packets": ${CLIENT_CNT},
  "quic_packets": ${QUIC_CNT},
  "notes": "This sanitized pcap contains extracted ClientHello and QUIC Initial (heuristic). Use tcprewrite to rewrite IP/MAC if needed."
}
JSON

echo "[*] Done. Sanitized PCAP: $SANITIZED_PCAP"
echo "[*] Metadata: $META_JSON"
echo "[*] To further anonymize addresses, install tcprewrite and run:"
echo "    tcprewrite --infile=${SANITIZED_PCAP} --outfile=${OUTPREFIX}.anonymized.pcap --seed=42 --enet-smac=00:00:00:00:00:00 --enet-dmac=00:00:00:00:00:00 --srcipmap=0.0.0.0/0:10.0.0.0/8 --dstipmap=0.0.0.0/0:10.0.0.0/8"

exit 0

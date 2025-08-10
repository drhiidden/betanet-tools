#!/usr/bin/env bash
# Example: run sanitizer on sample.pcap
set -euo pipefail

INPUT=sample.pcap
OUTPREFIX=sample

if [[ ! -f "$INPUT" ]]; then
  echo "Please place a sample pcap as $INPUT in the current dir to run this example."
  exit 1
fi

bash docs/templates/pcap_sanitizer.sh -i "$INPUT" -o "$OUTPREFIX"

echo "Sanitized pcap and metadata generated: ${OUTPREFIX}.sanitized.pcap, ${OUTPREFIX}.metadata.json"

# Quick inspect
if command -v tshark >/dev/null 2>&1; then
  echo "ClientHello packets in sanitized pcap:"
  tshark -r "${OUTPREFIX}.sanitized.pcap" -Y 'ssl.handshake.type == 1' -q -z io,phs
fi

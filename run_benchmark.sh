WORKLOAD_GEN="./target/release/tectonic-cli"
SPECS_DIR="specs"
OUTPUT_DIR="output"
LOG_DIR="logs"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$LOG_DIR"

WORKLOADS=(
  "phase_workload"
  # "interleave_wl"
  "varied_sortedness"
  # "kl"
  # "slowly_shifting"
  # "abruptly_shifting"
  # "varied_kv_length"
  # "hot_key"
  # "1m_i"
  # "hkr"

)

for workload in "${WORKLOADS[@]}"; do
  spec_path="$SPECS_DIR/$workload.spec.json"
  out_path="$OUTPUT_DIR/$workload"
  log_path="$LOG_DIR/${workload}_profile.log"

  echo ">>> Running $workload"
  echo "Output folder: $out_path"
  echo "Logging to: $log_path"

  mkdir -p "$out_path"

  /usr/bin/time -v "$WORKLOAD_GEN" generate -w "$spec_path" -o "$out_path" \
    > >(tee "$log_path") \
    2> >(tee -a "$log_path" >&2)

  if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
    echo "Got an error: workload $workload failed, cannot do it." | tee -a "$log_path"
  else
    echo ">>> Finished $workload"
  fi

  echo
done  
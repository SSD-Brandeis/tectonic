
WORKLOAD_GEN="./target/release/tectonic-cli"
SPECS_DIR="kvbench-specs"
OUTPUT_DIR="output_kvbenchscale"
LOG_DIR="logs_kvbenchscale"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$LOG_DIR"

WORKLOADS=(
  "i"
  "ii"
  "iii"
  "iv"
  "v"
 
)

for workload in "${WORKLOADS[@]}"; do
  spec_path="$SPECS_DIR/$workload.spec.json"
  out_path="$OUTPUT_DIR/$workload"
  log_path="$LOG_DIR/${workload}_profile.log"

  echo ">>> Running $workload"
  echo "Logging to: $log_path"

  mkdir -p "$out_path"

  /usr/bin/time -v "$WORKLOAD_GEN" generate -w "$spec_path" \
    > >(tee "$log_path") \
    2> >(tee -a "$log_path" >&2)

  if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
    echo "Got an error: workload $workload failed, cannot do it." | tee -a "$log_path"
  else
    echo ">>> Finished $workload"
    echo ">>> remove generated workload files "
   
    rm -rf "$out_path"
  fi

  echo
done

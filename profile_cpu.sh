
WORKLOAD_GEN="./target/release/tectonic-cli"
SPECS_DIR="specs"
OUTPUT_DIR="output"
LOG_DIR="logs"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$LOG_DIR"

WORKLOADS=(
  # "1m_i"
  # "hkr"
  "varied_sortedness"
  
)

for workload in "${WORKLOADS[@]}"; do
  spec_path="$SPECS_DIR/$workload.spec.json"
  out_path="$OUTPUT_DIR/$workload"
  log_path="$LOG_DIR/${workload}_profile.log"
  time_log_path="$LOG_DIR/${workload}_time.log"
  cpu_log_path="$LOG_DIR/${workload}_cpu.csv"

  echo ">>> Running $workload"
  echo "Output folder: $out_path"
  echo "Logging to: $log_path"
  echo "CPU log: $cpu_log_path"
  echo "Time profile: $time_log_path"

  mkdir -p "$out_path"


  "$WORKLOAD_GEN" generate -w "$spec_path" -o "$out_path" > >(tee "$log_path") 2> >(tee -a "$log_path" >&2) &
  pid=$!


  while ! ps -p $pid > /dev/null; do sleep 0.01; done

  pidstat 1 -h -r -u -p $pid > "$cpu_log_path" &
  monitor_pid=$!

  
  wait $pid
  exit_code=$?


  kill $monitor_pid 2>/dev/null


  /usr/bin/time -v -o "$time_log_path" true

  if [[ $exit_code -ne 0 ]]; then
    echo "sth is wrong: workload $workload failed." | tee -a "$log_path"
  else
    echo ">>> Finished $workload"
  fi

  echo
done

import os
import re
import matplotlib.pyplot as plt

tectonic_log_path = "/home/cc/tectonic/logs/insert_test_run_profile.log"
kvbench_log_path = "/home/cc/KV-WorkloadGenerator/ycsb_logs/Workload-F.log"

def extract_metrics(log_path):
    metrics = {
        "user_time": 0.0,
        "system_time": 0.0,
        "cpu_percent": 0.0,
        "max_res_mem_kb": 0
    }

    with open(log_path, 'r') as f:
        for line in f:
            if "User time (seconds):" in line:
                metrics["user_time"] = float(line.split(":")[1].strip())
            elif "System time (seconds):" in line:
                metrics["system_time"] = float(line.split(":")[1].strip())
            elif "Percent of CPU this job got:" in line:
                metrics["cpu_percent"] = float(line.split(":")[1].strip().replace('%', ''))
            elif "Maximum resident set size (kbytes):" in line:
                metrics["max_res_mem_kb"] = int(line.split(":")[1].strip())

    metrics["total_cpu_time"] = metrics["user_time"] + metrics["system_time"]
    return metrics

tectonic = extract_metrics(tectonic_log_path)
kvbench = extract_metrics(kvbench_log_path)

print("Tectonic Metrics:")
for k, v in tectonic.items():
    print(f"  {k}: {v}")
print("\nKVBench Metrics:")
for k, v in kvbench.items():
    print(f"  {k}: {v}")

labels = ["Tectonic", "KVBench"]

fig, axs = plt.subplots(1, 3, figsize=(16, 5))

axs[0].bar(labels, [tectonic["total_cpu_time"], kvbench["total_cpu_time"]], color=["blue", "orange"])
axs[0].set_title("Total CPU Time (s)")
axs[0].set_ylabel("Seconds")
# axs[0].set_ylim(0, 15) 

axs[1].bar(labels, [tectonic["cpu_percent"], kvbench["cpu_percent"]], color=["blue", "orange"])
axs[1].set_title("CPU Usage (%)")
axs[1].set_ylabel("Percentage")
# axs[1].set_ylim(0, 100) 

axs[2].bar(labels, [tectonic["max_res_mem_kb"], kvbench["max_res_mem_kb"]], color=["blue", "orange"])
axs[2].set_title("Max Resident Set Size (KB)")
axs[2].set_ylabel("KB")
# axs[2].set_ylim(0, 500000)  

# plt.suptitle("Tectonic vs KVBench")
# plt.tight_layout(rect=[0, 0, 1, 0.95])

plot_dir = "plots"
os.makedirs(plot_dir, exist_ok=True)
plot_path = os.path.join(plot_dir, "tectonic_vs_kvbench_metrics.png")
plt.savefig(plot_path)
print(f"\nPlot saved to: {plot_path}")

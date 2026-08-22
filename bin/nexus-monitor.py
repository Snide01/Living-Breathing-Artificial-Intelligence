import time, psutil, json
CONSTITUTION_HASH = "06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b"
def generate_nexus_log(node_id):
    cpu = psutil.cpu_percent(interval=0.1, percpu=True)
    ram = psutil.virtual_memory()
    return json.dumps({"node_id": node_id, "timestamp": int(time.time()), "evidence_hash": CONSTITUTION_HASH, "cpu_matrix_pct": cpu, "ram_pct": ram.percent, "fast_path_eligible": True, "warehouses_die_mycelium_lives": "87/70"}, indent=2)
print(generate_nexus_log("central_ohio_core_01"))

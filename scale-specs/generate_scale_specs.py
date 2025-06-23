import json

i_pct = 0.80
u_pct = 0.10
pq_pct = 0.10

baselines = [
    1_000_000,
    5_000_000,
    10_000_000,
    50_000_000,
    100_000_000,
]

for baseline in baselines:
    scale_str = f"{baseline // 1_000_000}".zfill(3)
    filename = f"{scale_str}m.spec.json"
    spec = {
        "$schema": "../workload_schema.json",
        "sections": [
            {
                "groups": [
                    {
                        "inserts": {
                            "amount": baseline,
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        }
                    },
                    {
                        "inserts": {
                            "amount": round(baseline * i_pct),
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        },
                        "updates": {
                            "amount": round(baseline * u_pct),
                            "val": {"uniform": {"len": 992}},
                        },
                        "point_queries": {"amount": round(baseline * pq_pct)},
                    },
                ]
            }
        ],
    }
    with open(filename, "w") as f:
        json.dump(spec, f, indent=2)

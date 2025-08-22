import json

i_pct = 0.80
u_pct = 0.10
pq_pct = 0.10


complex_i_pct = 0.35
complex_pq_pct = 0.40
complex_epq_pct = 0.10
complex_rq_pct = 0.10
complex_rd_pct = 0.04
complex_pd_pct = 0.01

baselines = [
    1_000_000,
    5_000_000,
    10_000_000,
    50_000_000,
    # 100_000_000,
]

for baseline in baselines:
    scale_str = f"{baseline // 1_000_000}".zfill(3)
    filename_simple = f"{scale_str}m.spec.json"
    filename_complex = f"{scale_str}m-complex.spec.json"
    spec_simple = {
        "$schema": "../workload_schema.json",
        "sections": [
            {
                "groups": [
                    {
                        "inserts": {
                            "op_count": baseline,
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        }
                    },
                    {
                        "inserts": {
                            "op_count": round(baseline * i_pct),
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        },
                        "updates": {
                            "op_count": round(baseline * u_pct),
                            "val": {"uniform": {"len": 992}},
                        },
                        "point_queries": {"op_count": round(baseline * pq_pct)},
                    },
                ]
            }
        ],
    }

    spec_complex = {
        "$schema": "../workload_schema.json",
        "sections": [
            {
                "groups": [
                    {
                        "inserts": {
                            "op_count": baseline,
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        }
                    },
                    {
                        "inserts": {
                            "op_count": round(baseline * complex_i_pct),
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                            "val": {"uniform": {"len": 992}},
                        },
                        "point_queries": {"op_count": round(baseline * complex_pq_pct)},
                        "empty_point_queries": {
                            "op_count": round(baseline * complex_epq_pct),
                            "key": {"uniform": {"len": 32, "character_set": "numeric"}},
                        },
                        "range_queries": {
                            "op_count": round(baseline * complex_rq_pct),
                            "selectivity": 0.1,
                        },
                        "point_deletes": {
                            "op_count": round(baseline * complex_pd_pct),
                        },
                        "range_deletes": {
                            "op_count": round(baseline * complex_rd_pct),
                            "selectivity": 0.01,
                        },
                    },
                ]
            }
        ],
    }
    with open(filename_simple, "w") as f:
        json.dump(spec_simple, f, indent=2)
    with open(filename_complex, "w") as f:
        json.dump(spec_complex, f, indent=2)

# Performance Improvements

Initial times

| Workload    | Time (s) | 
|-------------|---------:|
| 1m_i__1m_u  |   5.3982 |
| 1m_i__1m_d  |  3.4613* |
| 1m_i__1m_pq |   3.8674 |
| 1m_i__1m_rq | too long |

\* Did not generate correct, so not a valid comparison

## Switching to `String` to `Box<[u8]>`

Layout of a `String`.

```
+-----------------------+
| ptr 8 | len 8 | cap 8 |
+-+---------------------+
  |
+-v--------------------+
| utf8 rune 8-32 | ... |
+----------------------+
```

Layout of a `Box<[u8]>`.

```
+---------------+
| ptr 8 | len 8 |
+-+-------------+
  |
+-v------+-----+
| byte 8 | ... |
+--------+-----+
```

- also removed global operation vec in favor of passing a `impl Write`.
    - That way, a user could pass a `BufWriter`, `Vec`, or `sink()`

| Workload    | Time (s) |   Diff % |
|-------------|---------:|---------:|
| 1m_i__1m_u  |   3.6082 | -33.159% |
| 1m_i__1m_d  | too long |       -- |
| 1m_i__1m_pq |   2.4160 | -37.529% |
| 1m_i__1m_rq | too long |       -- |

- deletes cause a shift of the entire array, so it's too slow
    - use a `Vec<Option<Box<[u8]>>>` instead
- range queries + inserts require dynamic sorted keys, so it is just a slow operation

## Fixing correctness

New times, not comparable with older numbers

| Workload                        | Time (s) |
|---------------------------------|---------:|
| insert + update                 |   4.2759 |
| insert + delete                 |   1.5474 |
| insert + point query            |   2.8867 |
| insert + range query (even)     |   0.5243 |
| insert + range query (heavy i)  |   0.4805 |
| insert + range query (heavy rq) |   3.1251 |
| range query                     |   1.8457 |

| Workload                        | Time (s) |
|---------------------------------|---------:|
| insert + update                 |   3.9087 |
| insert + delete                 |   1.3311 |
| insert + point query            |   2.4533 |
| insert + range query (even)     |   0.5170 |
| insert + range query (heavy i)  |   0.5256 |
| insert + range query (heavy rq) |   3.9176 |
| range query                     |   1.6309 |

Get rid of `keys_sorted` and sort `keys_valid` during range queries.

| Workload                        | Time (s) |
|---------------------------------|---------:|
| insert + update                 |   3.7412 |
| insert + delete                 |   1.3253 |
| insert + point query            |   2.4357 |
| insert + range query (even)     |   0.2501 |
| insert + range query (heavy i)  |   0.0259 |
| insert + range query (heavy rq) |   0.1299 |
| range query                     |   1.4959 |

Implement a naive empty point query option. It is so slow, so the benchmark only has 100k inserts and 1k empty point
queries.

| Workload                           | Time (s) |
|------------------------------------|---------:|
| insert + update                    |   3.7798 |
| insert + delete                    |   1.3371 |
| insert + point query               |   2.4414 |
| insert + range query (even)        |   0.0249 |
| insert + range query (heavy i)     |   0.2585 |
| insert + range query (heavy rq)    |   0.1272 |
| range query                        |   1.3429 |
| empty point query (100k i, 1k eqp) |   0.2096 |

## More powerful key generation

No change

| Workload                           | Time (s) |
|------------------------------------|---------:|
| insert + update                    |   4.1403 |
| insert + delete                    |   1.4595 |
| insert + point query               |   2.7162 |
| insert + range query (even)        |   0.0270 |
| insert + range query (heavy i)     |   0.2880 |
| insert + range query (heavy rq)    |   0.1415 |
| range query                        |   1.5583 |
| empty point query (100k i, 1k eqp) |   0.2079 |
| **total benchmark time**           | 00:18:28 |


```json
{
  "key": {
    "segmented": {
      "separator": ";",
      "segments": [
        {
          "discrete": {
            "user": 0.6,
            "part": 0.4
          }
        },
        {
          "distribution": "uniform",
          "space": "Alphanumeric",
          "length": 16
        }
      ]
    }
  }
}
```

## TODO

look into:

- perf
- valgrind
    - cache grind
- io_stat

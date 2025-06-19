With uniform or insert order, the vec is already sorted. But with other sort metrics, every update, delete, pq, rq
requires a sort. Maybe in that case we could use something like a BTree.

things to look into

- Order Statistic Tree

| Operation           | Required Operations               |
|---------------------|-----------------------------------|
| Inserts             | Insert into keyset                |
| Updates             | Query random idx into keyset      |
| Deletes             | Delete random idx from keyset     |
| Range Deletes       | Delete a random range from keyset |
| Point Queries       | Query random idx into keyset      |
| Range Queries       | Query random idx into keyset      |
| Empty Point Queries | Contains check on keyset          |

| Data structure     | Insert   | Query random idx | Delete   | Delete Range | Contains | Notes                                                                                                          |
|--------------------|----------|------------------|----------|--------------|----------|----------------------------------------------------------------------------------------------------------------|
| Vec                | O(1)     | O(1)             | O(1)     | O(n)         | O(n)     |                                                                                                                |
| RandomKeySet       | O(1)     | O(1)             | O(1)     | O(n)         | O(1)     | Safe version requires O(2n) memory                                                                             |
| Vec + Bloom Filter | O(1)     | O(1)             | O(n)     | O(n)         | O(1)     |                                                                                                                |
| BTreeSet           | O(log n) | O(n/2)           | O(log n) | O(n)         | O(log n) | random idx needs to walk to set, so random distributions with more probable values at the front will be faster |

| Operation(s)                                                                 | Optimal Data Structure         | Explanation                                                                                  | Complexity: k keys, <op> operations |
|------------------------------------------------------------------------------|--------------------------------|----------------------------------------------------------------------------------------------|-------------------------------------|
| Inserts                                                                      | Vec<Option<Key>>               | O(1) to push onto a vec                                                                      | O(i)                                |
| Updates                                                                      | Vec<Option<Key>>               | Sort + filter on target metric, O(1) random index into vec.                                  | O(k log k + u)                      | 
| Deletes                                                                      | (invalid)                      |                                                                                              |                                     |
| Point Queries                                                                | Vec<Option<Key>>               | Sort + filter on target metric, and O(1) index into vec.                                     | O(k log k + pq)                     |
| Range Queries                                                                | Vec<Option<Key>>               | Sort + filter on target metric, and O(1) index into vec.                                     | O(k log k + rq)                     |
| Empty Point Queries                                                          | Vec<Option<Key>>, Bloom Filter | `Contains` check against bloom filter. FP rate should be proportional to key generation time | O(epq)                              |
| Inserts, Updates                                                             | Vec<Option<Key>>               |                                                                                              |                                     | 
| Inserts, Deletes                                                             | Vec<Option<Key>>               | i: O(1) push to vec. d: O(...) loop check for valid idx, set to None                         |                                     |
| Inserts, Point Queries                                                       | Vec<Option<Key>>               |                                                                                              |                                     |
| Inserts, Range Queries                                                       | Vec<Option<Key>>               |                                                                                              |                                     |
| Inserts, Empty Point Queries                                                 |                                |                                                                                              |                                     |
| Updates, Deletes                                                             |                                |                                                                                              |                                     |
| Updates, Point Queries                                                       |                                |                                                                                              |                                     |
| Updates, Range Queries                                                       |                                |                                                                                              |                                     |
| Updates, Empty Point Queries                                                 |                                |                                                                                              |                                     |
| Deletes, Point Queries                                                       |                                |                                                                                              |                                     |
| Deletes, Range Queries                                                       |                                |                                                                                              |                                     |
| Deletes, Empty Point Queries                                                 |                                |                                                                                              |                                     |
| Point Queries, Range Queries                                                 |                                |                                                                                              |                                     |
| Point Queries, Empty Point Queries                                           |                                |                                                                                              |                                     |
| Range Queries, Empty Point Queries                                           |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes                                                    |                                |                                                                                              |                                     |
| Inserts, Updates, Point Queries                                              |                                |                                                                                              |                                     |
| Inserts, Updates, Range Queries                                              |                                |                                                                                              |                                     |
| Inserts, Updates, Empty Point Queries                                        |                                |                                                                                              |                                     |
| Inserts, Deletes, Point Queries                                              |                                |                                                                                              |                                     |
| Inserts, Deletes, Range Queries                                              |                                |                                                                                              |                                     |
| Inserts, Deletes, Empty Point Queries                                        |                                |                                                                                              |                                     |
| Inserts, Point Queries, Range Queries                                        |                                |                                                                                              |                                     |
| Inserts, Point Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Inserts, Range Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Updates, Deletes, Point Queries                                              |                                |                                                                                              |                                     |
| Updates, Deletes, Range Queries                                              |                                |                                                                                              |                                     |
| Updates, Deletes, Empty Point Queries                                        |                                |                                                                                              |                                     |
| Updates, Point Queries, Range Queries                                        |                                |                                                                                              |                                     |
| Updates, Point Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Updates, Range Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Deletes, Point Queries, Range Queries                                        |                                |                                                                                              |                                     |
| Deletes, Point Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Deletes, Range Queries, Empty Point Queries                                  |                                |                                                                                              |                                     |
| Point Queries, Range Queries, Empty Point Queries                            |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Point Queries                                     |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Range Queries                                     |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Empty Point Queries                               |                                |                                                                                              |                                     |
| Inserts, Updates, Point Queries, Range Queries                               |                                |                                                                                              |                                     |
| Inserts, Updates, Point Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Inserts, Updates, Range Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Inserts, Deletes, Point Queries, Range Queries                               |                                |                                                                                              |                                     |
| Inserts, Deletes, Point Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Inserts, Deletes, Range Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Inserts, Point Queries, Range Queries, Empty Point Queries                   |                                |                                                                                              |                                     |
| Updates, Deletes, Point Queries, Range Queries                               |                                |                                                                                              |                                     |
| Updates, Deletes, Point Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Updates, Deletes, Range Queries, Empty Point Queries                         |                                |                                                                                              |                                     |
| Updates, Point Queries, Range Queries, Empty Point Queries                   |                                |                                                                                              |                                     |
| Deletes, Point Queries, Range Queries, Empty Point Queries                   |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Point Queries, Range Queries                      |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Point Queries, Empty Point Queries                |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Range Queries, Empty Point Queries                |                                |                                                                                              |                                     |
| Inserts, Updates, Point Queries, Range Queries, Empty Point Queries          |                                |                                                                                              |                                     |
| Inserts, Deletes, Point Queries, Range Queries, Empty Point Queries          |                                |                                                                                              |                                     |
| Updates, Deletes, Point Queries, Range Queries, Empty Point Queries          |                                |                                                                                              |                                     |
| Inserts, Updates, Deletes, Point Queries, Range Queries, Empty Point Queries |                                |                                                                                              |                                     |

---

Inserts, Updates, Point Deletes, Range Deletes, Point Queries, Range Queries, Empty Point Queries
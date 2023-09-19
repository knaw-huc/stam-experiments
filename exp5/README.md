## Query performance test

Note: TextFabric equivalent took 0.2s

Output (on proycon's system pollux):

```
A (naive, no variables):
15522 531.362247ms
A2 (collect all words):
15522 495.160168ms
A3 (normal):
15522 435.419596ms
A4 (parallel):
15522 33.879061ms
B:
890.533494ms
results: 15522
phrases considered: 253203
find_find_data_in_targets cumulative: 125.935956ms
find_data_about cumulative: 270.163553ms
test_data_about cumulative: 46.871339ms
annotations_by_data cumulative: 19.505611ms
intermezzo cumulative: 1.519039ms
result adding cumulative: 855.93µs
inner cumulative: 859.30609ms
inner2 cumulative: 720.013014ms
C (sets):
15522 1.158695658s
```

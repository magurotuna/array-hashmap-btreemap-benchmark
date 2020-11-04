# Result

```
running 1 test
test array::tests::test_binary_search ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

Gnuplot not found, using plotters backend
Benchmarking Search an element/Array/AbortController
Benchmarking Search an element/Array/AbortController: Warming up for 3.0000 s
Benchmarking Search an element/Array/AbortController: Collecting 100 samples in estimated 5.0000 s (1.9B iterations)
Benchmarking Search an element/Array/AbortController: Analyzing
Search an element/Array/AbortController
                        time:   [2.5694 ns 2.5711 ns 2.5729 ns]
                        change: [+1.7821% +1.8993% +2.0290%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking Search an element/ArrayBinarySearch/AbortController
Benchmarking Search an element/ArrayBinarySearch/AbortController: Warming up for 3.0000 s
Benchmarking Search an element/ArrayBinarySearch/AbortController: Collecting 100 samples in estimated 5.0010 s (19M iterations)
Benchmarking Search an element/ArrayBinarySearch/AbortController: Analyzing
Search an element/ArrayBinarySearch/AbortController
                        time:   [264.52 ns 264.72 ns 264.95 ns]
                        change: [-0.0670% +0.0223% +0.1149%] (p = 0.64 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/BTreeMap/AbortController
Benchmarking Search an element/BTreeMap/AbortController: Warming up for 3.0000 s
Benchmarking Search an element/BTreeMap/AbortController: Collecting 100 samples in estimated 5.0001 s (344M iterations)
Benchmarking Search an element/BTreeMap/AbortController: Analyzing
Search an element/BTreeMap/AbortController
                        time:   [14.568 ns 14.592 ns 14.616 ns]
                        change: [-2.8878% -2.5004% -2.1218%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking Search an element/HashMap/AbortController
Benchmarking Search an element/HashMap/AbortController: Warming up for 3.0000 s
Benchmarking Search an element/HashMap/AbortController: Collecting 100 samples in estimated 5.0000 s (234M iterations)
Benchmarking Search an element/HashMap/AbortController: Analyzing
Search an element/HashMap/AbortController
                        time:   [21.133 ns 21.213 ns 21.301 ns]
                        change: [-0.9799% -0.7505% -0.5417%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  13 (13.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/Array/MessageEvent
Benchmarking Search an element/Array/MessageEvent: Warming up for 3.0000 s
Benchmarking Search an element/Array/MessageEvent: Collecting 100 samples in estimated 5.0000 s (158M iterations)
Benchmarking Search an element/Array/MessageEvent: Analyzing
Search an element/Array/MessageEvent
                        time:   [31.465 ns 31.592 ns 31.724 ns]
                        change: [+0.1019% +0.4296% +0.7256%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
Benchmarking Search an element/ArrayBinarySearch/MessageEvent
Benchmarking Search an element/ArrayBinarySearch/MessageEvent: Warming up for 3.0000 s
Benchmarking Search an element/ArrayBinarySearch/MessageEvent: Collecting 100 samples in estimated 5.0010 s (20M iterations)
Benchmarking Search an element/ArrayBinarySearch/MessageEvent: Analyzing
Search an element/ArrayBinarySearch/MessageEvent
                        time:   [249.80 ns 249.91 ns 250.03 ns]
                        change: [+0.0011% +0.0750% +0.1511%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  2 (2.00%) high mild
Benchmarking Search an element/BTreeMap/MessageEvent
Benchmarking Search an element/BTreeMap/MessageEvent: Warming up for 3.0000 s
Benchmarking Search an element/BTreeMap/MessageEvent: Collecting 100 samples in estimated 5.0002 s (114M iterations)
Benchmarking Search an element/BTreeMap/MessageEvent: Analyzing
Search an element/BTreeMap/MessageEvent
                        time:   [43.537 ns 43.583 ns 43.633 ns]
                        change: [+1.5867% +1.7556% +1.9499%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/HashMap/MessageEvent
Benchmarking Search an element/HashMap/MessageEvent: Warming up for 3.0000 s
Benchmarking Search an element/HashMap/MessageEvent: Collecting 100 samples in estimated 5.0000 s (256M iterations)
Benchmarking Search an element/HashMap/MessageEvent: Analyzing
Search an element/HashMap/MessageEvent
                        time:   [19.533 ns 19.547 ns 19.561 ns]
                        change: [+2.1552% +2.2415% +2.3302%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
Benchmarking Search an element/Array/WritableStream
Benchmarking Search an element/Array/WritableStream: Warming up for 3.0000 s
Benchmarking Search an element/Array/WritableStream: Collecting 100 samples in estimated 5.0002 s (71M iterations)
Benchmarking Search an element/Array/WritableStream: Analyzing
Search an element/Array/WritableStream
                        time:   [69.891 ns 70.369 ns 70.806 ns]
                        change: [+10.640% +11.859% +13.022%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking Search an element/ArrayBinarySearch/WritableStream
Benchmarking Search an element/ArrayBinarySearch/WritableStream: Warming up for 3.0000 s
Benchmarking Search an element/ArrayBinarySearch/WritableStream: Collecting 100 samples in estimated 5.0008 s (20M iterations)
Benchmarking Search an element/ArrayBinarySearch/WritableStream: Analyzing
Search an element/ArrayBinarySearch/WritableStream
                        time:   [243.67 ns 244.04 ns 244.49 ns]
                        change: [-1.1258% -0.9632% -0.7534%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/BTreeMap/WritableStream
Benchmarking Search an element/BTreeMap/WritableStream: Warming up for 3.0000 s
Benchmarking Search an element/BTreeMap/WritableStream: Collecting 100 samples in estimated 5.0001 s (89M iterations)
Benchmarking Search an element/BTreeMap/WritableStream: Analyzing
Search an element/BTreeMap/WritableStream
                        time:   [55.680 ns 55.877 ns 56.091 ns]
                        change: [-0.4418% -0.2491% -0.0756%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) low severe
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/HashMap/WritableStream
Benchmarking Search an element/HashMap/WritableStream: Warming up for 3.0000 s
Benchmarking Search an element/HashMap/WritableStream: Collecting 100 samples in estimated 5.0000 s (256M iterations)
Benchmarking Search an element/HashMap/WritableStream: Analyzing
Search an element/HashMap/WritableStream
                        time:   [19.432 ns 19.440 ns 19.448 ns]
                        change: [-0.1275% -0.0174% +0.0870%] (p = 0.75 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
Benchmarking Search an element/Array/UNEXISTENT_ELEMENT
Benchmarking Search an element/Array/UNEXISTENT_ELEMENT: Warming up for 3.0000 s
Benchmarking Search an element/Array/UNEXISTENT_ELEMENT: Collecting 100 samples in estimated 5.0001 s (93M iterations)
Benchmarking Search an element/Array/UNEXISTENT_ELEMENT: Analyzing
Search an element/Array/UNEXISTENT_ELEMENT
                        time:   [53.695 ns 53.839 ns 53.990 ns]
                        change: [-3.8351% -3.3224% -2.8658%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
Benchmarking Search an element/ArrayBinarySearch/UNEXISTENT_ELEMENT
Benchmarking Search an element/ArrayBinarySearch/UNEXISTENT_ELEMENT: Warming up for 3.0000 s
Benchmarking Search an element/ArrayBinarySearch/UNEXISTENT_ELEMENT: Collecting 100 samples in estimated 5.0009 s (21M iterations)
Benchmarking Search an element/ArrayBinarySearch/UNEXISTENT_ELEMENT: Analyzing
Search an element/ArrayBinarySearch/UNEXISTENT_ELEMENT
                        time:   [243.63 ns 243.74 ns 243.85 ns]
                        change: [-0.2163% -0.1371% -0.0569%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
Benchmarking Search an element/BTreeMap/UNEXISTENT_ELEMENT
Benchmarking Search an element/BTreeMap/UNEXISTENT_ELEMENT: Warming up for 3.0000 s
Benchmarking Search an element/BTreeMap/UNEXISTENT_ELEMENT: Collecting 100 samples in estimated 5.0001 s (169M iterations)
Benchmarking Search an element/BTreeMap/UNEXISTENT_ELEMENT: Analyzing
Search an element/BTreeMap/UNEXISTENT_ELEMENT
                        time:   [29.457 ns 29.596 ns 29.747 ns]
                        change: [-3.3297% -2.9289% -2.5045%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking Search an element/HashMap/UNEXISTENT_ELEMENT
Benchmarking Search an element/HashMap/UNEXISTENT_ELEMENT: Warming up for 3.0000 s
Benchmarking Search an element/HashMap/UNEXISTENT_ELEMENT: Collecting 100 samples in estimated 5.0000 s (292M iterations)
Benchmarking Search an element/HashMap/UNEXISTENT_ELEMENT: Analyzing
Search an element/HashMap/UNEXISTENT_ELEMENT
                        time:   [17.287 ns 17.293 ns 17.300 ns]
                        change: [-0.3688% -0.2370% -0.1164%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
```


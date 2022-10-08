## Big picture
![](./doc/.excalidraw.svg)

## Bench tools
* https://ghz.sh/docs/install

* https://github.com/bheisler/criterion.rs


## JS interpreter

* https://github.com/boa-dev/boa
* https://github.com/HiRoFa/quickjs_es_runtime

## Grpc 
* https://github.com/hyperium/tonic


## Naive Boa bench (non release build)
```sh
 ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 10000 --rps 10000  --call jsfilter.Filter.filter -d '{"js":"(payload) => payload.a===\'x\'","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051
```

```sh

Summary:
  Count:        10000
  Total:        27.29 s
  Slowest:      56.46 ms
  Fastest:      16.97 ms
  Average:      26.89 ms
  Requests/sec: 366.42

Response time histogram:
  16.967 [1]    |
  20.917 [874]  |∎∎∎∎∎∎∎∎∎∎∎
  24.866 [2667] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  28.816 [2581] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  32.766 [3314] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  36.715 [339]  |∎∎∎∎
  40.665 [113]  |∎
  44.615 [75]   |∎
  48.564 [24]   |
  52.514 [10]   |
  56.464 [2]    |:

```


## Naive Quick bench  (non release build)

### don't reuse filter
```sh 
ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 10000 --rps 10000  --call jsfilter.Filter.filter -d '{"js":"function filter(payload){ return payload.a===\'x\'}","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051

```

```sh
Summary:
  Count:        10000
  Total:        9.77 s
  Slowest:      20.54 ms
  Fastest:      2.63 ms
  Average:      9.30 ms
  Requests/sec: 1023.89

Response time histogram:
  2.632  [1]    |
  4.422  [172]  |∎∎∎
  6.213  [1136] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  8.004  [2166] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  9.794  [2340] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  11.585 [2090] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  13.375 [1413] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  15.166 [526]  |∎∎∎∎∎∎∎∎∎
  16.956 [117]  |∎∎
  18.747 [30]   |∎
  20.538 [9]    |

```



### reuse filter
```sh 

cargo run --bin init-quick-js-bench

ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 10000 --rps 10000  --call jsfilter.Filter.isMatchingFilter -d '{"id":"1","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051

```

```sh
Summary:
  Count:        10000
  Total:        9.20 s
  Slowest:      20.45 ms
  Fastest:      2.37 ms
  Average:      8.75 ms
  Requests/sec: 1086.71

Response time histogram:
  2.373  [1]    |
  4.181  [232]  |∎∎∎∎
  5.989  [1362] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  7.797  [2457] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  9.605  [2237] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  11.413 [1916] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  13.221 [1267] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  15.029 [420]  |∎∎∎∎∎∎∎
  16.837 [78]   |∎
  18.645 [23]   |
  20.453 [7]    |

```
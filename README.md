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


## Naive Quick bench  

### don't reuse filter

#### Non release build 10k message
```sh 
ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 10000 --rps 10000  --call jsfilter.Filter.filter -d '{"js":"function filter(payload){ return payload.a===\'x\'}","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051

```
 #### Non release build 10k
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
#### release build 100k message

```sh 
ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 100000 --rps 20000  --call jsfilter.Filter.filter -d '{"js":"function filter(payload){ return payload.a===\'x\'}","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051

```

```sh

Summary:
  Count:        100000
  Total:        15.37 s
  Slowest:      27.15 ms
  Fastest:      0.17 ms
  Average:      1.14 ms
  Requests/sec: 6507.93

Response time histogram:
  0.166  [1]     |
  2.865  [99660] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  5.563  [268]   |
  8.262  [28]    |
  10.961 [22]    |
  13.659 [1]     |
  16.358 [3]     |
  19.057 [7]     |
  21.756 [0]     |
  24.454 [0]     |
  27.153 [10]    |
```


### reuse filter

#### non release build 10k message
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
#### release build
```sh 

cargo run --bin init-quick-js-bench

ghz --insecure --proto ./proto/js-filter.proto -c 10 -n 100000 --rps 20000  --call jsfilter.Filter.isMatchingFilter -d '{"id":"1","payload":"{\\"a\\":\\"x\\"}"}' 127.0.0.1:50051

```

```sh
Summary:
  Count:        100000
  Total:        13.31 s
  Slowest:      7.71 ms
  Fastest:      0.13 ms
  Average:      0.95 ms
  Requests/sec: 7512.75

Response time histogram:
  0.133 [1]     |
  0.890 [48446] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  1.647 [47016] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  2.405 [4160]  |∎∎∎
  3.162 [293]   |
  3.920 [69]    |
  4.677 [9]     |
  5.434 [4]     |
  6.192 [1]     |
  6.949 [0]     |
  7.706 [1]     |

```
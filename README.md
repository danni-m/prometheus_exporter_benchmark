Prometheus exporter load test
====


# golang
## Running
```
GOMAXPROCS=1  ./main
```

## Results
```
➜ wrk -d 30s -t 10 -c 20 http://localhost:8080/metrics
Running 30s test @ http://localhost:8080/metrics
  10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    57.82ms    9.63ms  94.17ms   67.13%
    Req/Sec    34.50      7.02    60.00     87.96%
  10395 requests in 30.10s, 1.75GB read
Requests/sec:    345.39
Transfer/sec:     59.55MB
```


# Rust
## Running
```
./target/release/prometheus_exporter_test
```

## Results
```
➜ wrk -d 30s -t 10 -c 20 http://localhost:8080/metrics
Running 30s test @ http://localhost:8080/metrics
  10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    35.36ms    1.85ms  53.93ms   89.72%
    Req/Sec    56.62      6.15    60.00     77.12%
  17002 requests in 30.10s, 3.47GB read
Requests/sec:    564.91
Transfer/sec:    118.18MB
```


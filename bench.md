# Simple benchmark

## macOS

| tool          | user | sys  | cpu  | total | rss   | rate       | ratio  |
|:-------------:|:----:|:----:|:----:|:-----:|:-----:|:----------:|:------:|
| (nop)         | -    | -    | -    | -     | -     | 3,748 MB/s | -      |
| native        | 7.7s | 3.9s |  92% | 12.6s |  5 MB | 1,358 MB/s | 36.7x  |
| wasi/wazero   | 13s  | 3.6s |  95% | 17.2s | 14 MB |   994 MB/s | 26.9x  |
| wasi/wasmer   | 27s  | 27 s |  94% | 57.3s | 34 MB |   298 MB/s |  8.1x  |
| wasi/wasmtime | 48s  | 71 s | 100% | 117 s | 36 MB |   146 MB/s |  4.0x  |
| (cut -b2-)    | 7.7m | 2.8s |  99% |  7.8m |  2 MB |    37 MB/s | (1.0x) |

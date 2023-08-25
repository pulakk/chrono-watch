# Chrono Watch (cwatch)
Chrono watch is a visual history tracker for data points across time. It listens to data through stdin and plots the history over time.

## Usage
Simply pipe any single floating point streaming output separated by new lines to cwatch
```bash
seq 1 100 | xargs -L 1 bash -c "echo \$RANDOM; sleep 0.5" | cwatch
```
```bash
Plot history
⡁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠈  ⡁ 29033.0
⠄                                                                     ⢸⠉⡇  ⠄
⠄                                                                     ⢸ ⡇  ⠄
⠂                                                                     ⢸ ⡇  ⠂
⡁                                                                     ⢸ ⡇  ⡁
⠄                                                                     ⢸ ⡇  ⠄
⠂                                                                     ⢸ ⡇  ⠂
⡁                                                                     ⢸ ⡇  ⡁
⠄                                                                     ⢸ ⡇  ⠄
⠂                                                                     ⢸ ⡇  ⠂
⡁                                                                     ⢸ ⣇⣀ ⡁
⠄                                                                     ⢸    ⠄
⠂                                                                    ⡤⠼    ⠂
⡁                                                                    ⡇     ⡁
⠄                                                                    ⡇     ⠄
⠂                                                                    ⡇     ⠂
⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁ 5835.0
-50.0                                                                 0.0
```

## Installation
Download latest binary for your specific OS from Github releases or build manually using `cargo build`.

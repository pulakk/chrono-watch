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
Download [latest pre-built binary](https://github.com/pulakk/chrono-watch/releases/latest/download/cwatch-linux-x86_64.tar.gz) for linux from Github releases.

```bash
# Install
wget -O - https://github.com/pulakk/chrono-watch/releases/latest/download/cwatch-linux-x86_64.tar.gz | tar -xvf -

# Run
./cwatch
```

You may build manually using `cargo build` for other OSes.

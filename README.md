# rust-commit2-worker

## run with cuda

```shell
# install cuda
# https://developer.nvidia.com/cuda-downloads?target_os=Linux&target_arch=x86_64&Distribution=Ubuntu&target_version=18.04&target_type=deb_network

wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
sudo apt-get update
sudo apt-get -y install cuda

export PATH=/usr/local/cuda/bin:$PATH
make runcuda
```

### 测试：

#### 编译原版：

```shell
cargo build --example test --release --features cuda
cp target/release/examples/test ./
```

#### 编译测试版

```shell
cargo build --example test --release --features cuda-supraseal
cp target/release/examples/test ./
```

编译后的文件在： target/release/examples/test

``` shell
TMPDIR=/mnt/lotus/tmp FIL_PROOFS_PARAMETER_CACHE=/var/tmp/filecoin-proof-parameters LOTUS_WORKER_SKIP_PARAM=true ./test --sector 1000 --sector-id 23 -f b810599428841dc5ccdd8d6a6ff9649c
```
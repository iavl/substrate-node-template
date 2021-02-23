#!/usr/bin/env bash

# 安装 subkey
curl https://getsubstrate.io -sSf | bash -s -- --fast
cargo install --force subkey --git https://github.com/paritytech/substrate --version 2.0.0 --locked

# 生成初始账户
export SECRET='wing race again asset legend decide swap box leader hold add symbol'
subkey inspect "$SECRET"

export SECRET='acid version ready inject impulse admit glove bird into lyrics burger receive'
subkey inspect "$SECRET"

export SECRET='remember spice empower cloud approve page praise pioneer wall lab grab sketch'
subkey inspect "$SECRET"

export SECRET='behave tower attend water faculty erupt that sad claw luggage message inquiry'
subkey inspect "$SECRET"

# 生成验证人账户和Session keys
for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
for i in 1 2 3 4; do for j in babe; do subkey inspect --scheme sr25519 "$SECRET//$i//$j"; done; done
for i in 1 2 3 4; do for j in grandpa; do subkey inspect --scheme ed25519 "$SECRET//$i//$j"; done; done
for i in 1 2 3 4; do for j in im_online; do subkey inspect --scheme sr25519 "$SECRET//$i//$j"; done; done

# 编译
cargo build --release

# 生成chain spec
./target/release/node-template build-spec --chain tao > customSpec.json

# 编码chain spec
./target/release/node-template build-spec --chain tao --raw > customSpec-raw.json
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
./target/release/node-template build-spec --chain tao-staking > tao-staking.json

./target/release/node-template build-spec --chain local > customSpec.json

# 编码chain spec
./target/release/node-template build-spec --chain tao-staking --raw > tao-staking-raw.json

./target/release/node-template build-spec --chain local --raw > customSpec-raw.json


	let endowed_accounts = vec![
		// 5CofS1P8bDL3kA1Xnwi7o1gWupMWEXSuCfLQUwiAtqDCrxxs
		hex!["20c363718239ce01110d0cec210fc623e008b58a7988c51a4d8734f2df28a17d"].into(),
		// 5CoXxz6YJg1rQ87zhRyUMX4oPAMvF6GuGZSKzgDgthypXVUw
		hex!["20aa4188baa85e057ceae411e9e8b1c6b1604977ff8c31a33bb38debb6abcc0b"].into(),
		// 5Dc8CVt6ivHLMhJP3uztcaZ3MRonmbLXCSBoZ2uACMKu8gad
		hex!["4432c87a5dd48cb456321be55f573119626ba04e9be732bd2c0b88f758efc960"].into(),
		// 5FLCHDZsnc47LVFPHTrcobcLmN7GpwkmAB3fGJfVUca3Ry9L
		hex!["908510084b9285192bddc815e8e3e64f1d572e011f3862a3d13d227c76233322"].into(),
	];


// 5G3koKYUuXVDtKQu1ZKQ3m2Dn5q6pC8hzWZDF53a5bR6gTWo
hex!["b03797cd606a7d2cc5f2eeab3068f662b8392236fbecae8a7a60940c0bbbb64a"].into(),
// 5H3n23ibheesCeGwfT6i1BnWNtvMWtHSgdSo2VJTXsseRY42
hex!["dc77ee70de928ff692521c2bca563d93268d28e3ba8333eaa958124e547dc458"].into(),
// 5CrM34VcUtzRHQ8dr9rVLUyciV5fbMjzgKsRrDGschf2sCa8
hex!["22cf331016e368f9aed28dda562b345178f4d7637f21b2c25382865209625670"].unchecked_into(),
// 5CKdn4ocmvNT8sCL92QEwkDAb4j92FKTQEQXN6V1UwofGw65
hex!["0b62f2e1068a89291659c248ff6c9a07a34f942e3ba8a4700d99a7fbe522caea"].unchecked_into(),
// 5ExmeSd3R2qJMGidNpTtsThzRvj4ahbBFW4gYLyRt3Gc84sS
hex!["802dfbc895211a247848e2efb2f253342279c473af42effbc9e2cc9a9a0d223e"].unchecked_into(),
),(
// 5DSZDLLL8EX6Ac2wsaMK3UDt7oEfSinPVRXAC9JFrr7KiEoe
hex!["3ce688f9bee25141c4a9d208310baf2314192851b3632780002e3bd187b97e55"].into(),
// 5CQ4rMKg1Kt4HteRom6QARLn8E5w1eZpVzUT3udTMa4FmfwC
hex!["0ec456285fdfd45773dd64b2503a8b6d23d1f7c2dcbb1bcdc028ecd7d4fd4c3a"].into(),
// 5FvgdaCP6PrTT11tiVU8csK8e8x7cNVzC7iTjQRWQaKasKpY
hex!["aad2d81e6229c13e42f87d3025781877a0b64b84ad1054e05272211ce768fc20"].unchecked_into(),
// 5CEhhW7yCUhkeMEQMeYuZvyPrMPGUxpboWk9kGzxVagXpzsb
hex!["079fec0c20ac5f58dd0e58ff1efa2cebd80e375052699e76e61bfaf6eeaf337e"].unchecked_into(),
// 5F9bgFpLxM1SGUXCbXmgYRcJK7fd9muUk89ihL34iPqRiHfB
hex!["8870200ff3df4b6724201eafc6daac4ca87ac1877b212bddd5644145a96f441c"].unchecked_into(),
),(
// 5DbwcHg2HpAPqHyMsuC6LqfqemxPpyTJDMb8XXqJtebMRDfK
hex!["440f2271f81bc962307eb1197502d1006dc9bae219012b7b6a2337658dee974c"].into(),
// 5EYx4Xp61cLwsvyJ5GrCy84uqG3sU9vRFi5qfn19gm6JqvDJ
hex!["6e0327ac31455fa6480febdd10fce00483dd6e37347001516b19fb9f25a80011"].into(),
// 5HTPbJgeVMf5Zt8NJ9AYvtKHbb6xdFrdyuuL4axMg3ymYZ3E
hex!["ee7a53955e8adf176fe4cbacd0c1c7439ad3bcafc463d8c3fb21b615bc99680b"].unchecked_into(),
// 5EtvdGs2YMG9q2nzodjRMwKQAxnfo9FhCno1s4w5REY7uBnC
hex!["7d3f3b1631e47eecd75afe7859038e4a267546eb81d691722ca2b7daee744d93"].unchecked_into(),
// 5EnknmfLckmo3N5wPxMvLgrMLuNCv378xHjVL8TPdRpmjVk8
hex!["788aa584aa75aa175936aca1acae4dd4a89e405de2bdf4b35949ef4fe26ef400"].unchecked_into(),
),(
// 5CkbPcFZazsz8NjZqupkoTjAfNsxBH8yUGFpU8RvEWzkBuDL
hex!["1e6c0c3d63e6ed885a18faa6394fc91ecfc9afdf028095178615ba4178c3e962"].into(),
// 5G3pt7fTCA9tQpRNrwD8q1Am9VFs8RfXgb92mEXFSGZ6cht2
hex!["b04556241709692db9e742fb8c44bf0da6dd55228dfb6bf496b4af292986955e"].into(),
// 5CDic61nnMNAUhV3h2dPA3ZnhLDQ98UceJf8zuBPgPSPLdPh
hex!["06dfba87c9abef7af04c5120003a50a04c1816df3749c54c6e3a53026267ff05"].unchecked_into(),
// 5GbGUHs5Givgg7k1qjg7hxH4AXscDKCiryCiLtEHCbus5fRd
hex!["c84015c7dd985f7406c4b5d013b4bb5fa9025f382090b3e28dc789609aa9a01b"].unchecked_into(),
// 5CXdpD5hv8gbcqWgjDNhhMkRFTWajNYbKMV1SPTksDieuZZ6
hex!["148a0499149297dd02c49866a7c9ee6b9060d15d970158c3913a4aec8ecc3708"].unchecked_into(),


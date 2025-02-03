#!/bin/sh

inputfile=/dev/urandom
count=16384
num_bytes2skip=1

geninput(){
	echo generating input random data...
	dd \
		if="${inputfile}" \
		of=./sample.d/rand.dat \
		bs=1048576 \
		count=${count} \
		status=progress
}

input(){
	cat ./sample.d/rand.dat
}

bench_nop(){
	input |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_native(){
	input |
		ENV_NUM_BYTES_TO_SKIP=${num_bytes2skip} \time -l ./rs-cut-lines-ascii |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_wazero(){
	input |
		\time -l \
			wazero \
				run \
				--env ENV_NUM_BYTES_TO_SKIP=${num_bytes2skip} \
				./rs-cut-lines-ascii.wasm |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_wasmtime(){
	input |
		\time -l \
			wasmtime \
				run \
				--env ENV_NUM_BYTES_TO_SKIP=${num_bytes2skip} \
				./rs-cut-lines-ascii.wasm |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_wasmer(){
	input |
		\time -l \
			wasmer \
				run \
				--env ENV_NUM_BYTES_TO_SKIP=${num_bytes2skip} \
				./rs-cut-lines-ascii.wasm |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

bench_cut(){
	input |
		\time -l cut -b2- /dev/stdin |
		dd \
			if=/dev/stdin \
			of=/dev/zero \
			bs=1048576 \
			status=progress
}

mkdir -p ./sample.d
test -f ./sample.d/rand.dat || geninput

#bench_nop
bench_native
#bench_wazero
#bench_wasmtime
#bench_wasmer
#bench_cut

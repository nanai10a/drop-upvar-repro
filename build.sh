#!/bin/bash

rustc +stage1 --edition 2024 --crate-type lib async-dep-by.rs
rustc +stage1 --edition 2024 --crate-type lib  sync-dep-by.rs


rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libasync_dep_by.rlib async-dep-to.rs -o async-dep-to-async.bin
rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libasync_dep_by.rlib  sync-dep-to.rs -o  sync-dep-to-async.bin

rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libsync_dep_by.rlib  async-dep-to.rs -o async-dep-to-sync.bin
rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libsync_dep_by.rlib   sync-dep-to.rs -o  sync-dep-to-sync.bin


echo " sync_drop /  sync_drop ..."
./sync-dep-to-sync.bin
echo

echo "async_drop /  sync_drop ..."
./async-dep-to-sync.bin
echo

echo " sync_drop / async_drop ..."
./sync-dep-to-async.bin
echo

echo "async_drop / async_drop ..."
./async-dep-to-async.bin

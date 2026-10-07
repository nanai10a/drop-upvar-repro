#!/bin/bash

rustc +stage1 --edition 2024 --crate-type lib async-dep-by.rs
rustc +stage1 --edition 2024 --crate-type lib  sync-dep-by.rs


rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libasync_dep_by.rlib async-dep-to.rs -o async-dep-to-async.bin
rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libasync_dep_by.rlib  sync-dep-to.rs -o  sync-dep-to-async.bin

rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libsync_dep_by.rlib  async-dep-to.rs -o async-dep-to-sync.bin
rustc +stage1 --edition 2024 --crate-type bin --extern dep_by=libsync_dep_by.rlib   sync-dep-to.rs -o  sync-dep-to-sync.bin


echo " sync /  sync ... $(./sync-dep-to-sync.bin)"
echo "async /  sync ... $(./async-dep-to-sync.bin)"
echo " sync / async ... $(./sync-dep-to-async.bin)"
echo "async / async ... $(./async-dep-to-async.bin)"

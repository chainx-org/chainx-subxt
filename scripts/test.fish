#!/usr/bin/env fish
cargo test -- --nocapture &| grep "type size registered" | sd '.*registered: \["(.*)".*' '$1'| sd '", "' '\n'| sort > a.log

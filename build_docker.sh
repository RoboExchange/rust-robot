#!/bin/bash 

cargo build --release 
strip target/release/rust-robot 
cp target/release/rust-robot docker 

(cd docker ; docker build . -t mah454/rust-robot:latest) 

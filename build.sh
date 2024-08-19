#!/bin/bash

echo "Windows向けx86プラグインをコンパイルします。"
cargo build --target=i686-pc-windows-gnu
echo "Windows向けx64プラグインをコンパイルします。"
cargo build --target=x86_64-pc-windows-gnu
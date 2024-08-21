#!/bin/bash

echo "Windows向けx86プラグインをコンパイルします。"
cargo build --target=i686-pc-windows-gnu --release

echo "Windows向けx64プラグインをコンパイルします。"
cargo build --target=x86_64-pc-windows-gnu --release

echo "出力物のコピーを行います。"
cp target/x86_64-pc-windows-gnu/release/bve_kobecitysubway_ats.dll ./output/ULine_x64.dll
cp ./output/ULine_x64.dll ./TestScenario/Vehicle/Ats/ULine_x64.dll
cp ./output/ULine_x64.dll /home/hir/Documents/BveTs/MyScenarios/U-Line/Vehicle/6000_1/Ats/ULine_x64.dll

cp target/i686-pc-windows-gnu/release/bve_kobecitysubway_ats.dll ./output/ULine_x86.dll
cp ./output/ULine_x86.dll ./TestScenario/Vehicle/Ats/ULine_x86.dll
cp ./output/ULine_x86.dll /home/hir/Documents/BveTs/MyScenarios/U-Line/Vehicle/6000_1/Ats/ULine_x86.dll

echo "すべて終了しました。"
# flash-algo

monazite 専用の [Flash Algorithm](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/flashAlgorithm.html) です。

## ビルド

`./update.sh` を実行すると、Flash Algorithm がビルドされ、`chip-descrpition.yaml` が更新されます。

## 3種類の Flash Algorithm について

書き込み先のバンクの違いによって3種類の Flash Algorithm があります。

### `monazite_bank1`

Bank1 に書き込むための Flash Algorithm です。

現在の SWAP_BANK がどちらであれ、SWAP_BANK=0 のときの Bank1 の領域（物理 Bank1）に書き込みます。

また、SWAP_BANK=0、NEXT_BOOT_BANK=0(Unchanged) を書き込みます。つまり、物理 Bank1 から起動するようにします。

### `monazite_bank2`

Bank2 に書き込むための Flash Algorithm です。

現在の SWAP_BANK がどちらであれ、SWAP_BANK=0 のときの Bank2 の領域（物理 Bank2）に書き込みます。

また、SWAP_BANK=1、NEXT_BOOT_BANK=0(Unchanged) を書き込みます。つまり、物理 Bank2 から起動するようにします。

### `monazite_mirrored`

Bank1 と Bank2 に同じデータを書き込むための Flash Algorithm です。

主にブートローダーを書き込む際に使用します。

SWAP_BANK の設定は変更しません。

# monazite

monazite は、[C2A core](https://github.com/arkedge/c2a-core) を用いて作られたフライトソフトウェアを NUCLEO-H753ZI 上で実行するプロジェクトです。

C2A を実行するためのランタイムや C2A HAL、軌道上でのアップデートを実現するブートローダなどが含まれます。

## プロジェクト構造

- monazite: NUCLEO-H753ZI 上で C2A を走らせるためのランタイム。C2A HAL を含む
- hal-bind: 各種 C2A HAL の Rust と C のバインディング
- c2a-example: monazite-rt を使った C2A のサンプルコード
  - sils: 主にソフトウェア開発用に、PC 上で実行するためのエントリポイント
  - monazite: 実機用のエントリポイント
- dev-hal
  - 上記 sils 向けの C2A HAL 実装。周辺ハードウェアをエミュレーションしている
- bootloader: ブートローダー
- flash-algo: 上記ブートローダーの仕様に整合する形でファームウェアを書き込むための flash algorithm

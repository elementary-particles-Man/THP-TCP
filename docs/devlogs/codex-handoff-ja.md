# CODEX作業引き継ぎ（日本語）

このファイルは後続CODEX向けの作業要約です。以降の応答は日本語で行うこと。

---

## 現状の成果物（主要）

### 仕様（Addendum/Spec）
- `docs/spec/THP-TCP-Draft.md`
- `docs/spec/THP-TCP-v1.0.1-Addendum.md`
- `docs/spec/THP-TCP-v1.0.2-Addendum.md`
- `docs/spec/THP-TCP-AESGCM-KeyMgmt-v1.0.md`
- `docs/spec/THP-TCP-CBOR-FieldTables-v1.0.md`
- `docs/spec/THP-TCP-Nickname-Addressing-v1.0.md`

### 機械可読テーブル
- `docs/spec/tables/thp-tcp-fixed-tokens-v1.0.1.json`
- `docs/spec/tables/thp-tcp-cbor-fields-v1.0.json`
- `docs/spec/tables/thp-tcp-nickname-fields-v1.0.json`

### ログ
- `logs/flux-zen-talk-log.txt`（会話・合意の経緯）
- `logs/thp-tcp-spec-input.txt`
- `docs/devlogs/gpt-5.2-analysis-request.txt`
- `docs/devlogs/gpt-analysis-summary.txt`

### README
- `README.md`（Phase0 v1.0.2のフレーム定義、仕様参照、開発方針）

---

## 重要な合意事項（確定）

### コア
- Bootstrap: `HELLO / DICT_SNAPSHOT / DICT_ACK`
- `max_datagram` 交渉（メッセージ無制限 / datagram上限あり）
- Fragmentation (CONT)
- AES-256-GCM nonce+tag 明示 on-wire
- CBOR deterministic（canonical）必須

### v1.0.2 追補で固定
- Phase0フレーム: `[TOKEN:u8][FLAGS:u8][LEN:u16be][PAYLOAD]`
- フラグ: `F_CONT=0x80`, `F_LAST=0x40`
- Fragment header: `[msg_id:16][part_no:u8][part_total:u8][orig_token:u8][frag_bytes...]`
- CONT=輸送上の分割継続、UPDATE=辞書内容更新（意味論分離）
- DICT_SNAPSHOTは必ず全片再構成後に処理。欠片はタイムアウト後ERROR。

---

## 直近の作業履歴（コミット）

- `c02f3c6`: Nickname Addressing v1.0追加 + 参照追記 + JSONテーブル
- `6a16579`: CBOR Field Tables v1.0追加 + v1.0.2参照
- `4b4ee97`: AES-GCM Key Mgmt v1.0追加 + v1.0.2参照
- `7fecd0b`: README改善 + 次論点合意記録
- `dca2647`: v1.0.2 Addendum追加
- `2bfd746`: 不要ファイルをOldsへ移動して最小構成化

---

## 実装（Rust）

- `src/thp_tcp_translator/` を新規作成済み（最小のヘッダcodec雛形あり）
- 以降は**Codexは監督役**として Zen/Gemini に実装タスクを投げ、検証のみ行う方針

---

## 注意点

- `logs/` は .gitignore 対象なので、必要なら `git add -f` で追加する。
- `.codex_backups/nickname_spec/*` がコミット済み（不要なら削除）
- `Olds/` に旧ファイルを退避済み

---

## 次の進め方（推奨）

1) **AES-GCM key mgmt / CBOR tables / nickname** の整合チェック
2) Rust translator の実装タスクを JSON化して Zen/Geminiへ委譲
3) GPT-5.2で仕様統合（英日）を生成

---

以上。

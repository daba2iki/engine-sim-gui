# Engine Sim GUI

`ange-yaghi/engine-sim` 用のエンジンパラメータを編集するための Windows 向け GUI アプリです。  
Tauri 2 / Vue 3 / TypeScript / Naive UI で実装し、網易雲音楽風の黄色テーマとカスタムタイトルバーを使います。

## 機能

- サンプル JSON を読み込んでパラメータ一覧を表示
- キー名・ラベル・説明で検索
- カテゴリ別の絞り込み
- 数値 / 文字列 / bool / select を編集
- 変更項目のハイライトと変更件数表示
- 保存 / リセット
- Tauri コマンドによる JSON 読み書き
- 自前タイトルバー（最小化 / 最大化 / 閉じる）

## 開発

```bash
npm install
npm run tauri:dev
```

ビルド:

```bash
npm run tauri:build
```

## ファイル構成

```text
engine-sim-gui/
├─ src/
│  ├─ App.vue
│  ├─ components/
│  │  ├─ TitleBar.vue
│  │  ├─ Sidebar.vue
│  │  └─ ParameterCard.vue
│  ├─ style.css
│  ├─ types.ts
│  └─ main.ts
├─ sample/
│  └─ sample.json
└─ src-tauri/
   ├─ src/main.rs
   ├─ capabilities/default.json
   └─ tauri.conf.json
```

## サンプル JSON

`sample/sample.json` は次の形式です。

```json
{
  "name": "engine-name",
  "parameters": [
    {
      "key": "starter_torque",
      "label": "スタータートルク",
      "type": "number",
      "value": 70,
      "default": 70,
      "unit": "lb·ft",
      "description": "セルがクランクを回す力。",
      "category": "基本"
    }
  ]
}
```

`type` は `number` / `string` / `bool` / `select` を使えます。  
`select` の場合は `options` に文字列配列を設定してください。

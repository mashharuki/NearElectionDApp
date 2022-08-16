# NearElectionDApp
Near上で動作するDApp開発用のリポジトリです。

#### トレイトとは

データ型を分類する仕組みのことです。

#### near SDK  Rust

NEARスマートコントラクトを作成するためのRustライブラリ

### 動かし方

```cmd
======================================================
👋 Welcome to NEAR! Learn more: https://docs.near.org/
🔧 Let's get your dApp ready.
======================================================
(NEAR collects anonymous information on the commands used. No personal information that could identify you is shared)


Creating a new NEAR dApp

======================================================
✅  Success! Created '/Users/harukikondo/git/NearElectionDApp/near-election-dapp/frontend'
   with a smart contract in Rust and a frontend template in React.js.
🦀 If you are new to Rust please visit https://www.rust-lang.org 

  Your next steps:
   - Navigate to your project:
         cd /Users/harukikondo/git/NearElectionDApp/near-election-dapp/frontend
   - Install all dependencies
         npm run deps-install
   - Test your contract in NEAR SandBox:
         npm test
   - Deploy your contract to NEAR TestNet with a temporary dev account:
         npm run deploy
   - Start your frontend:
         npm start

🧠 Read README.md to explore further.
```

#### ディレクトリ構成を出力する方法
 `tree -L 2 -F`  

 出力例  
 ```cmd
./
├── backend/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src/
│   └── target/
└── frontend/
    ├── README.md
    ├── contract/
    ├── frontend/
    ├── integration-tests/
    └── package.json
 ```

### Taliwindのインストールと設定ファイルを生成するコマンド
 `npm install -D tailwindcss postcss &&  npx tailwindcss init`  

#### 参考文献
 1. <a href="https://tailwindcss.jp"/>Tailwind</a>
 2. <a href="https://postcss.org/">PostCSS</a>
 3. <a href="https://docs.rs/near-sdk/latest/near_sdk/">near SDK</a>
# ♟️ Chess Fight Backend

Chess Fight 是一個基於 Rust 開發的自走棋遊戲後端專案，分為兩大子系統：

- ✅ **REST API**：使用 Axum 框架開發，實現回合邏輯與棋盤管理
- ✅ **WebSocket**：使用 tungstenite 提供即時通訊與指令處理機制，支援動作分派（如 ping、echo）

本專案展示如何使用模組化、強型別、可擴展的方式設計多人遊戲後端架構。

## 📦 專案結構

```
src/
├── main.rs              # 主程序入口
├── websocket/           # WebSocket 核心邏輯
│   └── mod.rs           # socket 接收與分派處理
├── handlers/            # 指令處理器
│   ├── mod.rs
│   ├── echo.rs
│   ├── ping.rs
│   └── unknown.rs
├── types/               # 資料模型（WsRequest / WsResponse）
│   ├── mod.rs
│   └── response.rs
├── router.rs            # WebSocket handler 註冊機制
└── (可擴充 axum/)
```

## 🚀 功能特色

- 支援 WebSocket 即時通訊
- 使用 trait-based handler 模式，擴展性高
- 資料格式統一（WsRequest / WsResponse）
- 未來可擴充 RESTful API（axum-ready 架構）
- 範例指令包含：ping、echo

## 🛠️ 開發需求

- Rust 1.70+
- Cargo 套件管理器
- 支援 WebSocket 的測試工具（如 websocat、瀏覽器 DevTools）

## 📦 安裝與執行

1. 安裝 Rust（https://www.rust-lang.org/tools/install）

2. Clone 此專案：
```bash
git clone https://github.com/AI-Driven-Creators/Chess_fight_backend.git
cd Chess_fight_backend
```

3. 編譯與執行：
```bash
cargo run
```

預設伺服器監聽位置為：`ws://127.0.0.1:9002`

## 📡 WebSocket 測試範例

### 使用 websocat

```bash
# 安裝
cargo install websocat

# 測試 echo
echo '{"action":"echo","data":{"msg":"hi"}}' | websocat ws://127.0.0.1:9002

# 測試 ping
echo '{"action":"ping"}' | websocat ws://127.0.0.1:9002
```

### 使用瀏覽器 DevTools

```javascript
const ws = new WebSocket("ws://127.0.0.1:9002");

ws.onopen = () => {
  ws.send(JSON.stringify({ action: "echo", data: { msg: "hello" } }));
  ws.send(JSON.stringify({ action: "ping" }));
};

ws.onmessage = (e) => console.log("Response:", e.data);
```

## 🧾 訊息格式

### 請求格式 (WsRequest)

```json
{
  "action": "echo",
  "data": {
    "msg": "Hello"
  }
}
```

### 回應格式 (WsResponse)

#### 成功：

```json
{
  "status": "ok",
  "data": {
    "echo": "Hello"
  },
  "error": null
}
```

#### 錯誤：

```json
{
  "status": "error",
  "data": null,
  "error": "unknown action"
}
```

## 🧪 測試與日誌

```bash
# 設定日誌等級
export RUST_LOG=info

# 執行測試
cargo test
```

建議每個 handler 寫一個單元測試，確保邏輯不被破壞。

## 🔧 未來擴展（推薦）

- ✅ 接入 axum 建立 REST API for user/login/game/record
- ✅ WebSocket 加入 auth middleware、session 管理
- ✅ 設計棋盤狀態管理模組
- ✅ 使用 tokio + async tungstenite 改為非同步處理

## 🙌 貢獻方式

1. Fork 專案
2. 創建新分支：`git checkout -b feature/my-feature`
3. 提交更動：`git commit -m 'Add my feature'`
4. 推送分支：`git push origin feature/my-feature`
5. 建立 Pull Request 🎉

## 📝 授權 License

本專案採用 MIT License
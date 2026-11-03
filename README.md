# 小 Claude 桌面版

呂世友老師的桌面小幫手：Claude 任務完成或有問題要問時，桌面上的小橘人會跳出泡泡、身後發光提醒。
做法沿用 quietpet-tauri（透明、無邊框、滑鼠穿透、不搶 PowerPoint 焦點）。

## 下載與使用
1. 下載 exe：https://github.com/tomogeo/xiao-claude-pet/raw/build-output/out/XiaoClaude.exe
2. 放到固定的資料夾（例如「文件\小Claude」），再雙擊開啟。開機自動啟動會記住這個位置。
3. 第一次開啟時 Windows 可能顯示「Windows 已保護您的電腦」：按「其他資訊」→「仍要執行」。

- 拖曳：按住小 Claude 拖到任何地方，位置會記住。
- 點一下：已讀（停止發光），會跳一下說「收到！」。
- 右鍵：上課模式（不出聲）、大小、立即檢查消息、暫時隱藏 10 分鐘、開機自動啟動、關閉。
- 系統匣（右下角時鐘旁）：顯示／隱藏、關閉。

## 消息怎麼來
小 Claude 每 90 秒讀一次 `pet-status` 分支的 `status.json`：

```json
{"state": "working | done | ask | idle", "message": "一句話摘要", "updatedAt": 1791797000000}
```

Claude 在任何對話裡要通知老師時，更新這個檔案即可（updatedAt 為毫秒時間戳，每次要變大）。
GitHub 的資料有約 1 分鐘快取，所以消息最慢約 2～3 分鐘出現。

## 結構
| 路徑 | 內容 |
|---|---|
| src/index.html | 畫面：小 Claude 動畫、泡泡、拖曳、讀狀態檔 |
| src-tauri/src/lib.rs | 主程式：視窗、滑鼠穿透、右鍵選單、系統匣、開機啟動、只開一隻 |
| src-tauri/tauri.conf.json | Tauri 設定（透明、無邊框、置頂、不顯示在工作列） |
| .github/workflows/build-windows.yml | 推到 main 時在 GitHub 雲端 Windows 打包 exe，放到 build-output 分支 |
| web/xiao-claude-web.html | 網頁版小 Claude（claude.ai Artifact）的原始檔備份 |
| tools/ci-status.sh | 查最新一次打包狀態 |

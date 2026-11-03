# 小 Claude 桌面版：給 Claude Code 的說明

呂世友老師（Tomo）的桌面通知小幫手。溝通用繁體中文、臺灣用字；老師不是工程師，說明少用術語、一步一步講。
動手前先問清楚、說明規劃，老師同意後再做。

- 架構與使用方式見 README.md。視窗與滑鼠穿透的做法來自 tomogeo/quietpet-tauri，改動前可以參考它。
- exe 由 GitHub Actions 在 Windows 上打包（推到 main 就會跑），查狀態用 tools/ci-status.sh。
- 這個 Linux 環境無法執行 Windows 版；Rust 可用 `cargo check --target x86_64-pc-windows-msvc` 先檢查，
  畫面可用 Chromium 開 src/index.html 測（攔截 api.github.com 回傳假的 status.json）。
- **不要動 pet-status 分支的歷史**：它只放 status.json，是小 Claude 讀消息的地方。

## 怎麼通知老師
更新 pet-status 分支的 status.json（例如用 GitHub 工具 create_or_update_file，帶上目前檔案的 sha）：
`{"state": "working" | "done" | "ask" | "idle", "message": "30 字內摘要", "updatedAt": 毫秒時間戳}`
同時也更新網頁版：https://claude.ai/artifact/GJyBbCrdMbLotJkQdzgWam 的資料庫（collection `pet`、doc `status`，同樣欄位）。

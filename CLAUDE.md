# 小 Claude 桌面版：給 Claude Code 的說明

呂世友老師（Tomo）的桌面通知小幫手。溝通用繁體中文、臺灣用字；老師不是工程師，說明少用術語、一步一步講。
動手前先問清楚、說明規劃，老師同意後再做。

- 架構與使用方式見 README.md。視窗與滑鼠穿透的做法來自 tomogeo/quietpet-tauri，改動前可以參考它。
- exe 由 GitHub Actions 在 Windows 上打包（推到 main 就會跑），查狀態用 tools/ci-status.sh。
- 這個 Linux 環境無法執行 Windows 版；Rust 可用 `cargo check --target x86_64-pc-windows-msvc` 先檢查，
  畫面可用 Chromium 開 src/index.html 測（攔截 api.github.com 回傳假的 status.json）。
- **不要動 pet-status 分支的歷史**：它只放 status.json，是小 Claude 讀消息的地方。

## 通知規則（2026-11 老師調整）
只在這些時候通知：預計超過 10 分鐘的工作「開始」(working) 與「做完」(done)；每次要問老師問題 (ask)。
小工作不通知。詳細寫法在老師的 claude.ai 個人偏好設定裡；status.json 在 pet-status 分支，網頁版是
https://claude.ai/artifact/GJyBbCrdMbLotJkQdzgWam（collection `pet`、doc `status`）。

# SEARU Studio
> **Universal Generative Design & R&D Suite**  
> 一個基於 Rust 的高性能跨領域生成式設計與物理聲學協同演化引擎。

---


## 📖 專案概述 (Overview)

**SEARU** 將工程約束求解與生成藝術轉化為高維參數空間的演化搜尋問題。透過模擬退火（Simulated Annealing）、Min-Max 對抗演算法與聲學/幾何/力學物理目標函數，自動尋找符合嚴格聲學和諧、空間利用率與結構剛度的最佳解。

---

## 🎛️ 4 大專屬獨立工作台 (Dedicated Studios)

SEARU Studio 2.5 配備專業的現代深石板色（Dark Slate）獨立工作頁面：

```
                    ┌─────────────────────────┐
                    │      SEARU Engine       │
                    └────────────┬────────────┘
         ┌───────────────────────┼───────────────────────┐
         ▼                       ▼                       ▼
┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│  ✨ Synesthesia  │    │  🎧 Music Studio │    │ 🏛️ Architecture  │
│ (3D CAD + Audio) │    │  (DSP & FFT 256) │    │ (2D SVG + 3D CAD)│
└──────────────────┘    └──────────────────┘    └──────────────────┘
                                 │
                                 ▼
                        ┌──────────────────┐
                        │ 💿 Album Release │
                        │ (Rayon 10-Track) │
                        └──────────────────┘
```

### 1. ✨ [Synesthesia (五感共鳴)](public/index.html)
- **4-D 設計意圖矩陣**：輸入侵略性 (`Aggression`)、優雅度 (`Elegance`)、結構密度 (`Density`) 與工業感 (`Industrialism`)。
- **跨感官同步生成**：一次點擊同時輸出 **3D 建築結構模型 (`.obj`)** 與 **44.1kHz 合成音訊 (`.wav`)**。
- **3D 互動視窗**：內建 `<model-viewer>` 360° 軌道檢視、自動旋轉與底座音訊同步播放器。

### 2. 🎧 [Music Studio (音樂工作室)](public/music.html)
- **古典對位法與調律**：SATB 四聲部平行禁忌判別，支援 12-TET 與純律（Just Intonation）。
- **聲學退火與 FM 合成**：基於 Plomp-Levelt 不和諧度曲線進行音高與 FM 諧波退火。
- **實時頻譜可視化**：256-FFT 60fps 動態頻譜儀，支援匯出標準 MIDI 與 32-bit Float WAV。

### 3. 🏛️ [Architecture (建築與城市)](public/architecture.html)
- **雙混沌空間演化**：AABB 0 碰撞約束、房間分區比例與動態風阻力矩對抗。
- **向量畫布**：高解析度 SVG 平面圖，支援滑鼠滾輪縮放 (Zoom) 與拖曳平移 (Pan)。
- **MegaCity 10 階段流水線**：協同演化空間 $\to$ 桁架力學 $\to$ PBR 材質，產出 3D 都市 CAD 與 CNC G-Code。

### 4. 💿 [Album Release (專輯發行)](public/album.html)
- **多執行緒平行生產**：調用 Rayon 多核心線程池，平行作曲、合成與渲染 10 軌完裝曲目。
- **發行目錄管理**：線上即時串流試聽、進度指示與批次資產下載。

---

## 🚀 快速上手 (Quick Start)

### 1. 啟動服務端

```bash
# 編譯並啟動 Axum 伺服器
cargo run --release
```

伺服器將在 `http://localhost:3000` 啟動 HTTP 服務與背景演化守護程序。

### 2. 開啟 Studio 工作台

使用瀏覽器前往：👉 **http://localhost:3000**

- 透過頂部導航列快速切換各領域專屬頁面。
- 支援全域快捷鍵：
  - <kbd>Space</kbd>：播放 / 暫停音訊
  - <kbd>Cmd / Ctrl + Enter</kbd>：立即觸發當前工作區生成計算

---

## 📜 授權協議 (License)

本專案採用 **SEARU 非商業用途源碼授權條款 (SEARU Non-Commercial Source License v1.0 / Based on PolyForm Noncommercial 1.0.0)**。

- ✅ **允許用途**：個人學習、學術研究、教育教學、非營利開源研究與個人評估。
- ❌ **嚴格禁止**：任何商業化營利行為、收費 SaaS/API 託管服務、商業產品內嵌或未經授權之商業資產轉售。
- 💼 **商業授權**：如需商業用途或企業級整合，請聯繫專案維護團隊取得書面授權。

詳細條款請參閱 [LICENSE](LICENSE)。


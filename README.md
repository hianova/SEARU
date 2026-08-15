# SEARU Studio: The Universal Generative Design & R&D Suite

SEARU 是一個以 Rust 構建的高性能**跨領域生成式設計與研發輔助引擎**。
它的目標是將複雜的工程與藝術設計問題轉化為參數空間的演化求解，透過模擬退火（Simulated Annealing）與約束對抗演算法，自動搜尋出符合聲學、幾何、力學與空間美學的最佳結構。

---

## ✨ 核心特色 (Core Highlights)

1. **真實聲學模型與巴哈對位法 (Music Theory & Acoustics)**：
   - 精確實現 Sethares (1993) 的 **Plomp-Levelt 聲學不和諧度數值積分**。
   - 嚴格判別 SATB 四聲部間的**平行五度與平行八度**禁忌，自動退火演化出嚴謹和諧的古典和弦進行。
   - 內建 DSP 弦波合成器與能量曲線宏觀編曲器，支援生成 WAV 與標準 MIDI 檔案。

2. **對抗式空間與建築佈局 (Architecture Layout Evolution)**：
   - 採用 Min-Max 雙混沌對抗模型（Dual-Chaos Co-Evolution）：建築候選者追求空間最大化與 0 重疊（AABB 碰撞懲罰），環境生成器則施加動態風阻力矩，自動演化出最堅固且空間利用率最高的室內/建築佈局。

3. **跨領域設計實驗場 (Multi-Domain Laboratories)**：
   - **力學結構 (Mechanics)**：2D 桁架拓撲受力分佈最佳化。
   - **電路走線 (PCB Routing)**：雙向交叉網格中繼點退火避障。
   - **視覺藝術 (Visual Art)**：基於 HSL 色彩和諧度與黃金比例的幾何向量生成。
   - **分形宇宙 (Fractal Multiverse)**：多層級遞迴同構向量畫布。

4. **平行化完整專輯流水線 (Batch Album Production)**：
   - 整合 `rayon` 多執行緒平行計算，一鍵批次作曲、合成、編曲並產出 10 首 3 分鐘音訊軌（含 WAV、MIDI 與專輯封面 SVG）。

---

## 🎛️ SEARU Studio 2.5 專業工作台

SEARU 配備了現代化的深石板色（Dark Slate）專業工作台 Web UI，提供：
- **8 大領域專屬工作區**（Music Studio, Architecture, Mechanics, PCB, Visual, MegaCity, Fractal, Album Release）。
- **即時 Web Audio 頻譜分析儀**（Realtime FFT Spectrum Visualizer）與音訊播放器。
- **可互動向量畫布 (Interactive SVG Viewport)**：支援滑鼠滾輪平滑縮放 (Zoom) 與拖曳平移 (Pan)。
- **模擬退火即時遙測 (Annealing Telemetry Monitor)**：即時繪製溫度降溫折線圖與 Loss 收斂過程。
- **一鍵多格式匯出**（WAV 音訊、MIDI 樂譜、SVG 向量圖、Profile JSON 設定檔）。

---

## 🚀 快速上手 (Getting Started)

### 1. 啟動伺服器

```bash
cargo run
```

伺服器將在 `http://localhost:3000` 啟動極速 Axum HTTP 服務與背景演化守護程序。

### 2. 開啟 Studio 工作台

打開瀏覽器，前往：
👉 **http://localhost:3000**

- 在上方切換領域工作區（如 `Music Studio`）。
- 調整左側參數或從頂部下拉選單選擇預設範本（如 *Baroque Bach Counterpoint*、*Cyberpunk MegaCity*）。
- 點擊 **Anneal & Synthesize** 即可即時試聽或預覽生成結果！

---

## 📡 REST API 端點一覽

| Method | Endpoint | 說明 |
| :--- | :--- | :--- |
| `POST / GET` | `/api/music/bach` | 傳入 Root 音高與小節數，退火計算巴哈進行並回傳 WAV 音訊 |
| `POST` | `/api/music/generate` | 傳入完整 ArtistProfile 進行能量曲線完整曲目生成 |
| `POST / GET` | `/api/architecture/floorplan` | 傳入密度、分區比例與風力，回傳空間佈局 SVG |
| `POST / GET` | `/api/mechanics/truss` | 回傳 2D 桁架拓撲受力分佈 SVG |
| `POST / GET` | `/api/pcb_routing/route` | 回傳 PCB 電路走線與焊盤 SVG |
| `POST / GET` | `/api/visual/art` | 傳入基礎色相與形狀數，回傳 HSL 幾何藝術 SVG |
| `POST` | `/api/megacity/pipeline` | 跨領域協同演化建築、力學與 PBR 材質，回傳都市藍圖 SVG |
| `POST / GET` | `/api/fractal/universe` | 遞迴擴展分形同構宇宙 SVG |
| `POST / GET` | `/api/album/release` | 平行生產 10 首完整專輯曲目 |
| `GET` | `/api/album/tracks` | 取得當前已發布專輯曲目清單 (JSON) |
| `GET` | `/api/album/track/:filename` | 串流/下載特定專輯 WAV、MIDI 或 SVG |
| `GET` | `/api/telemetry` | 透過 Server-Sent Events (SSE) 串流實時退火遙測數據 |

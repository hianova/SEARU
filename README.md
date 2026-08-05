# SEARU: The Universal Generative Design & R&D Suite

## 核心理念 (Core Philosophy)

SEARU 是一個通用的**「生成式設計與研發輔助引擎」**。
我們的終極目標是**「跨領域降低設計與研發的複雜度 (Reduce Design and R&D Complexity)」**。

無論是軟體架構、音樂藝術、視覺幾何，甚至是物理機構與材料力學，設計的本質往往是「在龐大的參數空間中尋找最優解」。傳統的研發依賴長時間的試錯與人力微調；SEARU 改變了這個範式：它將設計參數（和弦、幾何尺寸、物理邊界條件）視為「基因 (Genes)」。

透過強大核心引擎 (`The Crucible` 真正的模擬退火大腦)，SEARU 能夠在純粹的數學層面上，為任何領域「演化」與「計算」出最完美的結構。讓創作者與工程師從繁瑣的旋鈕海洋與盲目試驗中解放，專注於定義美學、物理邊界與高階約束。

## 為什麼需要 SEARU？

1. **從盲目試錯到數學收斂**：不管是尋找最和諧的巴哈對位法、最穩固的桁架結構，還是最精準的 PBR 反射率，SEARU 都能瞬間退火計算出符合邊界條件的最佳解。
2. **跨領域的研發大腦**：這套模擬退火與求解引擎可以毫無阻礙地套用在四大領域：`music` (音樂)、`visual` (視覺藝術)、`mechanics` (結構力學) 與 `materials` (光學材質)。
3. **藝術、工程與科學的交點**：將演算法化為人類創造力的最強延伸。

## 專案模組與四大生成領域

目前的 SEARU 包含一個核心科學大腦，並已成功打通四個數位設計實驗場域：

- `src/science/` (The Core Engine)
  - **核心演算大腦**：包含 `The Crucible` 模擬退火引擎。引擎具有溫度控制機制 (Cooling Schedule)，可以智慧地跳出局部最佳解，所有跨領域的演化都在這裡發生。
- `src/music/` (🎵 音樂領域)
  - **生成式音樂**：結合巴哈對位法法則與聲學距離，演化出平順且充滿張力的和弦進行 (Chord Progression)，並自帶 `SineSynth` 合成器與 Wav 匯出。
- `src/visual/` (🎨 視覺領域)
  - **視覺幾何與圖形**：使用 HSL 色彩和諧理論與形狀約束，演化出不相交疊、色彩協調的 SVG 幾何藝術，將參數化設計推向極致。
- `src/mechanics/` (🏗️ 力學領域)
  - **結構拓撲優化**：為 2D 桁架 (Truss) 系統尋找最佳的應力分佈與最小質量。在負載約束下演化出如生物骨骼般強韌的鋼架結構。
- `src/materials/` (🔮 材質領域)
  - **逆向物理渲染 (Inverse PBR)**：指定不同視角的目標顏色 (例如正面藍色、邊緣青色)，引擎能反推並計算出對應的 Albedo、Roughness、Metallic 參數。

## Web UI 互動儀表板

SEARU 現已進化為具備現代化 UI 的全端 Web 應用。我們使用 Rust `Axum` 打造極速的非同步 HTTP 伺服器，搭配深色 Glassmorphism (玻璃擬物化) 風格的前端，讓你可以直接在瀏覽器上進行四個領域的生成與即時預覽！

### 快速上手

啟動伺服器：

```bash
cargo run
```

接著打開瀏覽器，前往：
**http://localhost:3000**

- **🎵 Music Engine**: 點擊 Generate 即可在網頁直接播放生成的四和弦前衛進行。
- **🎨 Visual Engine**: 點擊 Generate 即時顯示演化出的幾何 SVG 藝術。
- **🏗️ Mechanics Engine**: 點擊 Generate 查看拓撲優化的鋼管結構佈局。
- **🔮 Materials Engine**: 點擊 Generate 觀看 JSON 格式回傳的完美光學匹配參數。

享受把數學法則化為設計藝術的極致體驗！

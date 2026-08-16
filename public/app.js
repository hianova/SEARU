/**
 * ==========================================================================
 * SEARU Studio v2.5 - Core Multi-Page Client Application
 * Handles Synesthesia 3D, Music Studio DSP, Architecture Viewport & Album Catalog
 * ==========================================================================
 */

// --- State Management ---
const studioState = {
    activePage: document.body.dataset.page || 'synesthesia',
    currentAudioBlob: null,
    currentSvgString: null,
    currentMidiBlob: null,
    currentProfile: {
        culture: {
            tuning: "12-TET",
            phrase_length_bars: 4,
            rhythmic_grid: "4/4"
        },
        physics: {
            dissonance_tolerance: 2.0,
            fractal_chaos: 5.0
        }
    },
    // Viewport transform state
    zoom: 1.0,
    panX: 0,
    panY: 0,
    isPanning: false,
    panStartX: 0,
    panStartY: 0,
    // Telemetry history
    telemetryHistory: []
};

// --- DOM Elements Cache ---
const DOM = {
    // Top Controls
    topExportProfileBtn: document.getElementById('exportProfileBtn'),
    presetSelect: document.getElementById('presetSelect'),
    
    // Viewport elements
    viewportTitle: document.getElementById('viewportTitle'),
    viewportTag: document.getElementById('viewportTag'),
    svgStageWrapper: document.getElementById('svgStageWrapper'),
    svgStage: document.getElementById('svgStage'),
    synesthesiaStageWrapper: document.getElementById('synesthesiaStageWrapper'),
    audioStageWrapper: document.getElementById('audioStageWrapper'),
    canvasControls: document.getElementById('canvasControls'),
    synControls: document.getElementById('synControls'),
    synModelViewer: document.getElementById('synModelViewer'),
    synAudioPlayer: document.getElementById('synAudioPlayer'),
    btnAutoRotate: document.getElementById('btnAutoRotate'),
    btnDownloadObj: document.getElementById('btnDownloadObj'),
    btnDownloadWav: document.getElementById('btnDownloadWav'),
    synDockTitle: document.getElementById('synDockTitle'),
    synDockMeta: document.getElementById('synDockMeta'),
    
    // Audio Player elements
    audio: document.getElementById('globalAudio'),
    playBtn: document.getElementById('playerPlayBtn'),
    playIcon: document.getElementById('playIcon'),
    pauseIcon: document.getElementById('pauseIcon'),
    seekSlider: document.getElementById('playerSeekSlider'),
    volSlider: document.getElementById('playerVolumeSlider'),
    curTimeText: document.getElementById('playerCurrentTime'),
    totTimeText: document.getElementById('playerTotalTime'),
    trackTitle: document.getElementById('playerTrackTitle'),
    trackDetails: document.getElementById('playerTrackDetails'),
    spectrumCanvas: document.getElementById('spectrumCanvas'),
    
    // Telemetry elements
    telemetryCanvas: document.getElementById('telemetryCanvas'),
    statTemp: document.getElementById('statTemp'),
    statLoss: document.getElementById('statLoss'),
    statIters: document.getElementById('statIters'),
    statRate: document.getElementById('statRate'),
    
    // Console elements
    consoleBody: document.getElementById('consoleBody'),
    clearConsoleBtn: document.getElementById('btnClearConsole'),
    
    // Export buttons
    exportWavBtn: document.getElementById('btnExportWav'),
    exportMidiBtn: document.getElementById('btnExportMidi'),
    exportSvgBtn: document.getElementById('btnExportSvg'),
    exportDnaBtn: document.getElementById('btnExportDna'),
    
    // Synesthesia Controls
    btnGenSyn: document.getElementById('btnGenerateSynesthesia'),
    synSpinner: document.getElementById('synSpinner'),
    synAggressionSlider: document.getElementById('synAggressionSlider'),
    synEleganceSlider: document.getElementById('synEleganceSlider'),
    synDensitySlider: document.getElementById('synDensitySlider'),
    synIndustrialismSlider: document.getElementById('synIndustrialismSlider'),
    synAggressionVal: document.getElementById('synAggressionVal'),
    synEleganceVal: document.getElementById('synEleganceVal'),
    synDensityVal: document.getElementById('synDensityVal'),
    synIndustrialismVal: document.getElementById('synIndustrialismVal'),
    
    // ScriptGo Elements
    tabUiControls: document.getElementById('tabUiControls'),
    tabScriptGo: document.getElementById('tabScriptGo'),
    uiControlsSection: document.getElementById('uiControlsSection'),
    scriptGoSection: document.getElementById('scriptGoSection'),
    sgoTemplateSelect: document.getElementById('sgoTemplateSelect'),
    sgoEditor: document.getElementById('sgoEditor'),
    chkHotLoad: document.getElementById('chkHotLoad'),
    btnBakeScript: document.getElementById('btnBakeScript'),
    btnRunScript: document.getElementById('btnRunScript'),
    scriptSpinner: document.getElementById('scriptSpinner'),
    
    // Domain action buttons
    btnGenMusic: document.getElementById('btnGenerateMusic'),
    btnGenFm: document.getElementById('btnGenerateFm'),
    fmDissonanceSlider: document.getElementById('fmDissonanceSlider'),
    fmDissonanceVal: document.getElementById('fmDissonanceVal'),
    musicRootSelect: document.getElementById('musicRootSelect'),
    musicChordsSlider: document.getElementById('musicChordsSlider'),
    musicDissonanceSlider: document.getElementById('musicDissonanceSlider'),
    btnGenArch: document.getElementById('btnGenerateArch'),
    btnGenTruss: document.getElementById('btnGenerateTruss'),
    btnGenMegaCity: document.getElementById('btnGenerateMegaCity'),
    megaDensitySlider: document.getElementById('megaDensitySlider'),
    btnTriggerAlbum: document.getElementById('btnTriggerAlbum'),
    albumCard: document.getElementById('albumTracklistCard'),
    albumTrackList: document.getElementById('albumTrackList'),
    
    // Zoom controls
    btnZoomIn: document.getElementById('btnZoomIn'),
    btnZoomOut: document.getElementById('btnZoomOut'),
    btnResetView: document.getElementById('btnResetView')
};

// --- Toast System ---
function showToast(msg, type = 'info') {
    const container = document.getElementById('toastContainer');
    if (!container) return;
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.innerText = msg;
    container.appendChild(toast);
    setTimeout(() => {
        toast.style.opacity = '0';
        toast.style.transform = 'translateY(8px)';
        toast.style.transition = 'all 0.3s ease';
        setTimeout(() => toast.remove(), 300);
    }, 3200);
}

// --- Logger Utility ---
function log(msg, type = 'info') {
    if (!DOM.consoleBody) return;
    const line = document.createElement('div');
    line.className = `log-line ${type}`;
    const time = new Date().toLocaleTimeString();
    line.innerText = `[${time}] > ${msg}`;
    DOM.consoleBody.appendChild(line);
    DOM.consoleBody.scrollTop = DOM.consoleBody.scrollHeight;
}

DOM.clearConsoleBtn?.addEventListener('click', () => {
    if (DOM.consoleBody) {
        DOM.consoleBody.innerHTML = '';
        log('Console cleared.', 'sys');
    }
});

// --- Web Audio & Spectrum Analyzer ---
let audioCtx = null;
let analyser = null;
let audioSource = null;
let isAudioInitialized = false;

function initWebAudio() {
    if (isAudioInitialized || !DOM.audio) return;
    try {
        const AudioContext = window.AudioContext || window.webkitAudioContext;
        audioCtx = new AudioContext();
        analyser = audioCtx.createAnalyser();
        analyser.fftSize = 256;
        analyser.smoothingTimeConstant = 0.8;
        
        audioSource = audioCtx.createMediaElementSource(DOM.audio);
        audioSource.connect(analyser);
        analyser.connect(audioCtx.destination);
        
        isAudioInitialized = true;
        drawSpectrum();
        log('Web Audio DSP pipeline connected to AnalyserNode.', 'sys');
    } catch (e) {
        console.warn('Web Audio init:', e);
    }
}

function drawSpectrum() {
    if (!analyser || !DOM.spectrumCanvas) return;
    const canvas = DOM.spectrumCanvas;
    const ctx = canvas.getContext('2d');
    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    
    function render() {
        requestAnimationFrame(render);
        analyser.getByteFrequencyData(dataArray);
        
        ctx.fillStyle = '#020617';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Background grid lines
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.04)';
        ctx.lineWidth = 1;
        for (let x = 0; x < canvas.width; x += 40) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x, canvas.height);
            ctx.stroke();
        }
        
        const barWidth = (canvas.width / bufferLength) * 1.6;
        let x = 0;
        
        for (let i = 0; i < bufferLength; i++) {
            const barHeight = (dataArray[i] / 255) * (canvas.height - 30);
            
            const gradient = ctx.createLinearGradient(0, canvas.height, 0, canvas.height - barHeight);
            gradient.addColorStop(0, 'rgba(56, 189, 248, 0.2)');
            gradient.addColorStop(0.7, '#38bdf8');
            gradient.addColorStop(1, '#34d399');
            
            ctx.fillStyle = gradient;
            ctx.fillRect(x, canvas.height - barHeight, barWidth - 1, barHeight);
            
            x += barWidth;
            if (x > canvas.width) break;
        }
    }
    render();
}

// --- Audio Player Controls ---
DOM.playBtn?.addEventListener('click', async () => {
    initWebAudio();
    if (audioCtx && audioCtx.state === 'suspended') {
        await audioCtx.resume();
    }
    
    if (DOM.audio.paused) {
        if (!DOM.audio.src) {
            log('No audio loaded. Generating track...', 'info');
            generateFullMusic();
            return;
        }
        DOM.audio.play();
    } else {
        DOM.audio.pause();
    }
});

DOM.audio?.addEventListener('play', () => {
    if (DOM.playIcon) DOM.playIcon.style.display = 'none';
    if (DOM.pauseIcon) DOM.pauseIcon.style.display = 'block';
});

DOM.audio?.addEventListener('pause', () => {
    if (DOM.playIcon) DOM.playIcon.style.display = 'block';
    if (DOM.pauseIcon) DOM.pauseIcon.style.display = 'none';
});

DOM.audio?.addEventListener('timeupdate', () => {
    if (!isNaN(DOM.audio.duration) && DOM.audio.duration > 0) {
        const perc = (DOM.audio.currentTime / DOM.audio.duration) * 100;
        if (DOM.seekSlider) DOM.seekSlider.value = perc;
        if (DOM.curTimeText) DOM.curTimeText.innerText = formatTime(DOM.audio.currentTime);
        if (DOM.totTimeText) DOM.totTimeText.innerText = formatTime(DOM.audio.duration);
    }
});

DOM.seekSlider?.addEventListener('input', (e) => {
    if (DOM.audio && !isNaN(DOM.audio.duration)) {
        DOM.audio.currentTime = (parseFloat(e.target.value) / 100) * DOM.audio.duration;
    }
});

DOM.volSlider?.addEventListener('input', (e) => {
    if (DOM.audio) {
        DOM.audio.volume = parseFloat(e.target.value);
    }
});

function formatTime(sec) {
    if (isNaN(sec)) return '0:00';
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return `${m}:${s < 10 ? '0' : ''}${s}`;
}

function loadAndPlayAudioBlob(blob, title, details) {
    initWebAudio();
    studioState.currentAudioBlob = blob;
    const url = URL.createObjectURL(blob);
    if (DOM.audio) {
        DOM.audio.src = url;
        DOM.audio.load();
        DOM.audio.play().catch(e => console.log('Autoplay prevented:', e));
    }
    
    if (DOM.trackTitle) DOM.trackTitle.innerText = title;
    if (DOM.trackDetails) DOM.trackDetails.innerText = details;
    log(`Synthesized: ${title} (${(blob.size / 1024).toFixed(1)} KB)`, 'success');
    showToast(`Synthesized ${title}`, 'success');
}

// --- SVG Interactive Viewport (Pan & Zoom) ---
function updateSvgTransform() {
    if (DOM.svgStage) {
        DOM.svgStage.style.transform = `translate(${studioState.panX}px, ${studioState.panY}px) scale(${studioState.zoom})`;
    }
}

DOM.btnZoomIn?.addEventListener('click', () => {
    studioState.zoom = Math.min(5.0, studioState.zoom * 1.25);
    updateSvgTransform();
});

DOM.btnZoomOut?.addEventListener('click', () => {
    studioState.zoom = Math.max(0.2, studioState.zoom / 1.25);
    updateSvgTransform();
});

DOM.btnResetView?.addEventListener('click', () => {
    studioState.zoom = 1.0;
    studioState.panX = 0;
    studioState.panY = 0;
    updateSvgTransform();
});

DOM.svgStageWrapper?.addEventListener('wheel', (e) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    studioState.zoom = Math.max(0.2, Math.min(5.0, studioState.zoom * delta));
    updateSvgTransform();
}, { passive: false });

DOM.svgStageWrapper?.addEventListener('mousedown', (e) => {
    studioState.isPanning = true;
    studioState.panStartX = e.clientX - studioState.panX;
    studioState.panStartY = e.clientY - studioState.panY;
});

window.addEventListener('mousemove', (e) => {
    if (!studioState.isPanning) return;
    studioState.panX = e.clientX - studioState.panStartX;
    studioState.panY = e.clientY - studioState.panStartY;
    updateSvgTransform();
});

window.addEventListener('mouseup', () => {
    studioState.isPanning = false;
});

function displaySvg(svgString, title = 'Generated Visual Blueprint') {
    studioState.currentSvgString = svgString;
    if (DOM.svgStage) {
        DOM.svgStage.innerHTML = svgString;
    }
    
    studioState.zoom = 1.0;
    studioState.panX = 0;
    studioState.panY = 0;
    updateSvgTransform();
    
    log(`Rendered SVG: ${title}`, 'success');
    showToast(`Rendered ${title}`, 'success');
}

// --- Telemetry Chart Engine ---
function renderTelemetryChart(lossCurve = null) {
    if (!DOM.telemetryCanvas) return;
    const canvas = DOM.telemetryCanvas;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    
    ctx.clearRect(0, 0, w, h);
    
    // Grid
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
    ctx.lineWidth = 1;
    for (let i = 0; i < w; i += 30) {
        ctx.beginPath(); ctx.moveTo(i, 0); ctx.lineTo(i, h); ctx.stroke();
    }
    for (let i = 0; i < h; i += 20) {
        ctx.beginPath(); ctx.moveTo(0, i); ctx.lineTo(w, i); ctx.stroke();
    }
    
    // Draw Simulated Annealing cooling curve
    ctx.beginPath();
    ctx.moveTo(0, 10);
    
    const points = lossCurve || generateMockAnnealingCurve();
    for (let i = 0; i < points.length; i++) {
        const x = (i / (points.length - 1)) * (w - 20) + 10;
        const y = h - 15 - (points[i] * (h - 30));
        ctx.lineTo(x, Math.max(5, Math.min(h - 5, y)));
    }
    
    ctx.strokeStyle = '#38bdf8';
    ctx.lineWidth = 2;
    ctx.stroke();
}

function generateMockAnnealingCurve() {
    const pts = [];
    let val = 1.0;
    for (let i = 0; i < 40; i++) {
        val *= 0.92;
        val += (Math.random() - 0.45) * 0.05 * val;
        pts.push(Math.max(0.01, val));
    }
    return pts;
}

renderTelemetryChart();

// --- Real-time SSE Telemetry Receiver ---
function initSseTelemetry() {
    try {
        const evtSource = new EventSource('/api/telemetry');
        evtSource.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.loss !== undefined && DOM.statLoss) {
                    DOM.statLoss.innerText = data.loss.toFixed(4);
                }
                if (data.temperature !== undefined && DOM.statTemp) {
                    DOM.statTemp.innerText = `${data.temperature.toFixed(4)}°`;
                }
                if (data.iterations !== undefined && DOM.statIters) {
                    DOM.statIters.innerText = data.iterations.toLocaleString();
                }
            } catch (err) {}
        };
    } catch (e) {
        console.warn('SSE Telemetry:', e);
    }
}
initSseTelemetry();

// --- Value Slider Badges Sync ---
function bindSlider(sliderId, badgeId, formatter) {
    const slider = document.getElementById(sliderId);
    const badge = document.getElementById(badgeId);
    if (!slider || !badge) return;
    slider.addEventListener('input', (e) => {
        badge.innerText = formatter(e.target.value);
    });
}

bindSlider('synAggressionSlider', 'synAggressionVal', v => parseFloat(v).toFixed(2));
bindSlider('synEleganceSlider', 'synEleganceVal', v => parseFloat(v).toFixed(2));
bindSlider('synDensitySlider', 'synDensityVal', v => parseFloat(v).toFixed(2));
bindSlider('synIndustrialismSlider', 'synIndustrialismVal', v => parseFloat(v).toFixed(2));

bindSlider('musicChordsSlider', 'musicChordsVal', v => `${v} Chords`);
bindSlider('musicDissonanceSlider', 'musicDissonanceVal', v => parseFloat(v).toFixed(1));
bindSlider('fmDissonanceSlider', 'fmDissonanceVal', v => parseFloat(v).toFixed(2));

bindSlider('archDensitySlider', 'archDensityVal', v => `${v} Rooms`);
bindSlider('archZoningSlider', 'archZoningVal', v => `${Math.round(v * 100)}% Commercial`);
bindSlider('archWindSlider', 'archWindVal', v => `${parseFloat(v).toFixed(1)} F`);

bindSlider('trussForceSlider', 'trussForceVal', v => `${v} N`);
bindSlider('megaDensitySlider', 'megaDensityVal', v => `${v} Rooms`);

// Tuning Toggle
const btnTuning12 = document.getElementById('tuning12Tet');
const btnTuningJust = document.getElementById('tuningJust');
if (btnTuning12 && btnTuningJust) {
    btnTuning12.addEventListener('click', () => {
        btnTuning12.classList.add('active');
        btnTuningJust.classList.remove('active');
        studioState.currentProfile.culture.tuning = "12-TET";
    });
    btnTuningJust.addEventListener('click', () => {
        btnTuningJust.classList.add('active');
        btnTuning12.classList.remove('active');
        studioState.currentProfile.culture.tuning = "Just Intonation";
    });
}

// MegaColor Picker
const colorPicker = document.getElementById('megaColorPicker');
const colorVal = document.getElementById('megaColorVal');
if (colorPicker && colorVal) {
    colorPicker.addEventListener('input', (e) => {
        colorVal.innerText = e.target.value;
    });
}

// 3D Auto Rotate Toggle
DOM.btnAutoRotate?.addEventListener('click', () => {
    const viewer = DOM.synModelViewer;
    if (!viewer) return;
    if (viewer.hasAttribute('auto-rotate')) {
        viewer.removeAttribute('auto-rotate');
        DOM.btnAutoRotate.classList.remove('active');
    } else {
        viewer.setAttribute('auto-rotate', '');
        DOM.btnAutoRotate.classList.add('active');
    }
});

// --- Synesthesia Intent Presets ---
const presets = {
    cyber: { aggression: 0.85, elegance: 0.20, density: 0.70, industrialism: 0.90 },
    organic: { aggression: 0.10, elegance: 0.90, density: 0.40, industrialism: 0.10 },
    scifi: { aggression: 0.50, elegance: 0.70, density: 0.80, industrialism: 0.85 },
    minimal: { aggression: 0.20, elegance: 0.80, density: 0.30, industrialism: 0.40 }
};

function applyIntentPreset(p) {
    if (DOM.synAggressionSlider) DOM.synAggressionSlider.value = p.aggression;
    if (DOM.synAggressionVal) DOM.synAggressionVal.innerText = p.aggression.toFixed(2);
    if (DOM.synEleganceSlider) DOM.synEleganceSlider.value = p.elegance;
    if (DOM.synEleganceVal) DOM.synEleganceVal.innerText = p.elegance.toFixed(2);
    if (DOM.synDensitySlider) DOM.synDensitySlider.value = p.density;
    if (DOM.synDensityVal) DOM.synDensityVal.innerText = p.density.toFixed(2);
    if (DOM.synIndustrialismSlider) DOM.synIndustrialismSlider.value = p.industrialism;
    if (DOM.synIndustrialismVal) DOM.synIndustrialismVal.innerText = p.industrialism.toFixed(2);
}

document.getElementById('presetCyber')?.addEventListener('click', (e) => {
    document.querySelectorAll('.preset-pills .pill-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    applyIntentPreset(presets.cyber);
});
document.getElementById('presetOrganic')?.addEventListener('click', (e) => {
    document.querySelectorAll('.preset-pills .pill-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    applyIntentPreset(presets.organic);
});
document.getElementById('presetSciFi')?.addEventListener('click', (e) => {
    document.querySelectorAll('.preset-pills .pill-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    applyIntentPreset(presets.scifi);
});
document.getElementById('presetMinimal')?.addEventListener('click', (e) => {
    document.querySelectorAll('.preset-pills .pill-btn').forEach(b => b.classList.remove('active'));
    e.target.classList.add('active');
    applyIntentPreset(presets.minimal);
});

// --- API Execution Handlers ---

// 0. Synesthesia Generator
DOM.btnGenSyn?.addEventListener('click', async () => {
    log('Initiating Synesthesia 5-Sense Co-Evolution...', 'info');
    DOM.btnGenSyn.disabled = true;
    if (DOM.synSpinner) DOM.synSpinner.style.display = 'inline-block';
    DOM.btnGenSyn.querySelector('.btn-text').innerText = 'OPTIMIZING 3D & AUDIO...';
    
    try {
        const payload = {
            aggression: parseFloat(DOM.synAggressionSlider.value),
            elegance: parseFloat(DOM.synEleganceSlider.value),
            density: parseFloat(DOM.synDensitySlider.value),
            industrialism: parseFloat(DOM.synIndustrialismSlider.value)
        };
        
        const res = await fetch('/api/synesthesia', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        
        if (!res.ok) {
            const errData = await res.json().catch(() => ({ message: 'Server error' }));
            throw new Error(errData.message || 'Synesthesia generation failed');
        }
        
        const data = await res.json();
        const timestamp = Date.now();
        
        // 3D Model Refresh
        if (DOM.synModelViewer) {
            DOM.synModelViewer.src = `/release/synesthesia.obj?t=${timestamp}`;
        }
        
        // Audio Refresh
        if (DOM.synAudioPlayer) {
            DOM.synAudioPlayer.src = `/release/synesthesia.wav?t=${timestamp}`;
            DOM.synAudioPlayer.play().catch(e => console.warn('Autoplay prevented:', e));
        }
        
        if (DOM.btnDownloadObj) DOM.btnDownloadObj.href = `/release/synesthesia.obj?t=${timestamp}`;
        if (DOM.btnDownloadWav) DOM.btnDownloadWav.href = `/release/synesthesia.wav?t=${timestamp}`;
        
        if (DOM.synDockTitle) DOM.synDockTitle.innerText = `Synesthesia Experience (Agg: ${payload.aggression} • Eleg: ${payload.elegance})`;
        if (DOM.synDockMeta) DOM.synDockMeta.innerText = `Density: ${payload.density} • Ind: ${payload.industrialism} • Bohlen-Pierce / 12-TET`;
        
        renderTelemetryChart();
        log('Synesthesia Experience generated and synchronized successfully!', 'success');
        showToast('Synesthesia 3D + Audio generated!', 'success');
        
    } catch (e) {
        log(`Generation Error: ${e.message}`, 'err');
        showToast(`Error: ${e.message}`, 'error');
    } finally {
        DOM.btnGenSyn.disabled = false;
        DOM.synSpinner.style.display = 'none';
        DOM.btnGenSyn.querySelector('.btn-text').innerText = 'GENERATE SYNESTHESIA ✨';
    }
});

// ScriptGo Tab Toggles
DOM.tabUiControls?.addEventListener('click', () => {
    DOM.tabUiControls.classList.add('active');
    DOM.tabScriptGo.classList.remove('active');
    if (DOM.uiControlsSection) DOM.uiControlsSection.style.display = 'flex';
    if (DOM.scriptGoSection) DOM.scriptGoSection.style.display = 'none';
});

DOM.tabScriptGo?.addEventListener('click', () => {
    DOM.tabScriptGo.classList.add('active');
    DOM.tabUiControls.classList.remove('active');
    if (DOM.scriptGoSection) DOM.scriptGoSection.style.display = 'flex';
    if (DOM.uiControlsSection) DOM.uiControlsSection.style.display = 'none';
});

// ScriptGo execution
DOM.btnRunScript?.addEventListener('click', async () => {
    DOM.btnRunScript.disabled = true;
    DOM.scriptSpinner.style.display = 'inline-block';
    DOM.btnRunScript.querySelector('.btn-text').innerText = 'EXECUTING SCRIPT...';
    
    try {
        const payload = { script: DOM.sgoEditor.value };
        const response = await fetch('/api/script/run', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        
        if (!response.ok) {
            const errText = await response.text();
            throw new Error(errText);
        }
        
        const data = await response.json();
        
        // Update DOM elements just like normal generation
        const bust = `?v=${Date.now()}`;
        DOM.synModelViewer.src = data.architecture_obj + bust;
        DOM.synAudioPlayer.src = data.music_wav + bust;
        DOM.btnDownloadObj.href = data.architecture_obj + bust;
        DOM.btnDownloadWav.href = data.music_wav + bust;
        
        // Log register states to console
        const {r1, r2, r3, r4} = data.registers;
        log(`Script Output Registers: R1=${r1}(Agg), R2=${r2}(Ele), R3=${r3}(Den), R4=${r4}(Ind)`, 'sys');
        
        // Optionally update the sliders visually to match script intent
        DOM.synAggressionSlider.value = r1 / 100.0;
        DOM.synEleganceSlider.value = r2 / 100.0;
        DOM.synDensitySlider.value = r3 / 100.0;
        DOM.synIndustrialismSlider.value = r4 / 100.0;
        DOM.synAggressionVal.innerText = (r1 / 100.0).toFixed(2);
        DOM.synEleganceVal.innerText = (r2 / 100.0).toFixed(2);
        DOM.synDensityVal.innerText = (r3 / 100.0).toFixed(2);
        DOM.synIndustrialismVal.innerText = (r4 / 100.0).toFixed(2);
        
        showToast('ScriptGo Executed: Synesthesia Generated!');
        DOM.synAudioPlayer.play().catch(e => console.log('Auto-play prevented', e));
        
    } catch (err) {
        log(`Script Error: ${err.message}`, 'error');
        showToast('Script Execution Failed', 'error');
    } finally {
        DOM.btnRunScript.disabled = false;
        DOM.scriptSpinner.style.display = 'none';
        DOM.btnRunScript.querySelector('.btn-text').innerText = 'EXECUTE SCRIPTGO ⚡';
    }
});

// Template Selector
const TEMPLATES = {
    cyber: `// SearuScript: Cyber Brutalist
let aggression: Float = 95.0;
let elegance: Float = 5.0;
let density: Float = 90.0;
let industrialism: Float = 80.0;`,
    organic: `// SearuScript: Harmonic Organic
let aggression: Float = 10.0;
let elegance: Float = 95.0;
let density: Float = 30.0;
let industrialism: Float = 10.0;`,
    glitch: `// SearuScript: Maximum Entropy (Glitch)
let aggression: Float = 100.0;
let elegance: Float = 0.0;
let density: Float = 100.0;
let industrialism: Float = 100.0;`
};
DOM.sgoTemplateSelect?.addEventListener('change', (e) => {
    if (TEMPLATES[e.target.value]) {
        DOM.sgoEditor.value = TEMPLATES[e.target.value];
        if (DOM.chkHotLoad && DOM.chkHotLoad.checked) {
            DOM.btnRunScript.click();
        }
    }
});

// Auto Hot-Load (Debounced)
let hotLoadTimer = null;
DOM.sgoEditor?.addEventListener('input', () => {
    if (DOM.chkHotLoad && DOM.chkHotLoad.checked) {
        clearTimeout(hotLoadTimer);
        hotLoadTimer = setTimeout(() => {
            if (!DOM.btnRunScript.disabled) {
                DOM.btnRunScript.click();
            }
        }, 300);
    }
});

// Bake & Freeze
let isBaked = false;
DOM.btnBakeScript?.addEventListener('click', () => {
    isBaked = !isBaked;
    if (isBaked) {
        DOM.sgoEditor.disabled = true;
        DOM.sgoEditor.style.opacity = '0.5';
        DOM.chkHotLoad.checked = false;
        DOM.chkHotLoad.disabled = true;
        DOM.btnBakeScript.innerText = '🔓 Unbake State';
        DOM.btnBakeScript.style.background = '#2a9d8f';
        DOM.sgoTemplateSelect.disabled = true;
        showToast('Generative DNA Baked & Frozen!', 'success');
        log('Generative state baked. Scripts locked.', 'sys');
    } else {
        DOM.sgoEditor.disabled = false;
        DOM.sgoEditor.style.opacity = '1';
        DOM.chkHotLoad.disabled = false;
        DOM.btnBakeScript.innerText = '🔒 Bake & Freeze';
        DOM.btnBakeScript.style.background = '#a83232';
        DOM.sgoTemplateSelect.disabled = false;
        showToast('Generative DNA Unlocked.');
    }
});

// DNA Export
DOM.exportDnaBtn?.addEventListener('click', () => {
    const text = DOM.sgoEditor ? DOM.sgoEditor.value : '';
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'intent_dna.sgo';
    a.click();
    URL.revokeObjectURL(url);
    showToast('ScriptGo DNA Exported!');
    log('Exported intent_dna.sgo to disk', 'sys');
});

// 1. Music (Full Composition & FM)
async function generateFullMusic() {
    if (!DOM.btnGenMusic) return;
    DOM.btnGenMusic.disabled = true;
    DOM.btnGenMusic.innerText = 'ANNEALING VOICES...';
    log('Igniting The Crucible: Annealing musical score with Plomp-Levelt acoustics...', 'info');
    
    const profile = {
        culture: {
            tuning: studioState.currentProfile.culture.tuning,
            phrase_length_bars: parseInt(DOM.musicChordsSlider?.value || '4'),
            rhythmic_grid: "4/4"
        },
        physics: {
            dissonance_tolerance: parseFloat(DOM.musicDissonanceSlider?.value || '2.0'),
            fractal_chaos: 5.0
        }
    };
    
    try {
        const res = await fetch('/api/music/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(profile)
        });
        
        if (res.ok) {
            const blob = await res.blob();
            loadAndPlayAudioBlob(blob, `Generative Composition (${profile.culture.phrase_length_bars} Bars)`, `Tuning: ${profile.culture.tuning} • Diss: ${profile.physics.dissonance_tolerance}`);
            renderTelemetryChart();
        } else {
            const err = await res.json().catch(() => ({ message: 'Error generating audio' }));
            log(`Error: ${err.message}`, 'err');
            showToast(err.message, 'error');
        }
    } catch (e) {
        log(`Network error: ${e.message}`, 'err');
        showToast(e.message, 'error');
    } finally {
        DOM.btnGenMusic.disabled = false;
        DOM.btnGenMusic.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg> GENERATE FULL COMPOSITION`;
    }
}
DOM.btnGenMusic?.addEventListener('click', generateFullMusic);

async function generateFmAudio() {
    if (!DOM.btnGenFm) return;
    DOM.btnGenFm.disabled = true;
    DOM.btnGenFm.innerText = 'SYNTHESIZING...';
    log('Igniting The Crucible: Annealing FM parameters to target dissonance...', 'info');
    
    const diss = parseFloat(DOM.fmDissonanceSlider?.value || '0.5');
    
    try {
        const res = await fetch(`/api/music/fm?dissonance=${diss}`, { method: 'GET' });
        if (res.ok) {
            const blob = await res.blob();
            loadAndPlayAudioBlob(blob, `Dialectical FM Timbre`, `Target Dissonance: ${diss.toFixed(2)}`);
            renderTelemetryChart();
        } else {
            log('Error generating FM audio.', 'err');
            showToast('Error generating FM audio', 'error');
        }
    } catch (e) {
        log(`Network error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenFm.disabled = false;
        DOM.btnGenFm.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 12h-4l-3 9L9 3l-3 9H2"></path></svg> SYNTHESIZE FM TIMBRE`;
    }
}
DOM.btnGenFm?.addEventListener('click', generateFmAudio);

// 2. Architecture (Floorplan)
DOM.btnGenArch?.addEventListener('click', async () => {
    DOM.btnGenArch.disabled = true;
    DOM.btnGenArch.innerText = 'EVOLVING SPATIAL BLUEPRINT...';
    log('Running Dual-Chaos Min-Max Room Evolution against Wind Force...', 'info');
    
    const density = parseInt(document.getElementById('archDensitySlider')?.value || '20');
    const zoning_ratio = parseFloat(document.getElementById('archZoningSlider')?.value || '0.5');
    const max_wind_force = parseFloat(document.getElementById('archWindSlider')?.value || '50');
    
    try {
        const res = await fetch('/api/arch/floorplan', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ density, zoning_ratio, max_wind_force })
        });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, `Architecture Layout (${density} Rooms)`);
            renderTelemetryChart();
        } else {
            const err = await res.json().catch(() => ({ message: 'Error generating floorplan' }));
            showToast(err.message, 'error');
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenArch.disabled = false;
        DOM.btnGenArch.innerText = 'EVOLVE 2D FLOORPLAN (SVG)';
    }
});

// 3. Mechanics (Truss)
DOM.btnGenTruss?.addEventListener('click', async () => {
    DOM.btnGenTruss.disabled = true;
    DOM.btnGenTruss.innerText = 'SOLVING TOPOLOGY...';
    log('Optimizing 2D Truss load paths and minimizing mass...', 'info');
    
    try {
        const res = await fetch('/api/mechanics/truss', { method: 'GET' });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, '2D Truss Topology Statics');
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenTruss.disabled = false;
        DOM.btnGenTruss.innerText = 'OPTIMIZE 2D TRUSS MECHANICS';
    }
});

// 4. MegaCity Co-Evolution
DOM.btnGenMegaCity?.addEventListener('click', async () => {
    DOM.btnGenMegaCity.disabled = true;
    DOM.btnGenMegaCity.innerText = 'RUNNING 10-STAGE PIPELINE...';
    log('Executing MegaCity Pipeline: Arch -> Truss -> Material -> Stress -> CAD...', 'info');
    
    const density = parseInt(DOM.megaDensitySlider?.value || '25');
    const hex = colorPicker ? colorPicker.value : '#38bdf8';
    const r = parseInt(hex.slice(1, 3), 16) / 255;
    const g = parseInt(hex.slice(3, 5), 16) / 255;
    const b = parseInt(hex.slice(5, 7), 16) / 255;
    
    const payload = {
        arch: { density, zoning_ratio: 0.6, max_wind_force: 60.0 },
        visual: { fractal_depth: 12, base_hue: 200.0 },
        mechanics: { target_r: r, target_g: g, target_b: b }
    };
    
    try {
        const res = await fetch('/api/megacity/pipeline', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        if (res.ok) {
            const result = await res.json();
            log(`MegaCity Pipeline Completed: ${result.message}`, 'success');
            showToast('MegaCity 3D CAD & G-Code Exported!', 'success');
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenMegaCity.disabled = false;
        DOM.btnGenMegaCity.innerText = 'RUN 10-STAGE MEGACITY CAD PIPELINE';
    }
});

// 5. Album Release & Catalog
DOM.btnTriggerAlbum?.addEventListener('click', async () => {
    DOM.btnTriggerAlbum.disabled = true;
    DOM.btnTriggerAlbum.innerText = 'SPAWNING RAYON WORKERS...';
    log('Initiating Rayon Parallel Album Pipeline (10 tracks)...', 'info');
    
    try {
        const res = await fetch('/api/album/release', { method: 'POST' });
        if (res.ok) {
            const data = await res.json();
            log(`Success: ${data.message}`, 'success');
            showToast('Album production started!', 'success');
            let polls = 0;
            const poller = setInterval(async () => {
                polls++;
                await fetchAlbumTracklist();
                if (polls > 8) clearInterval(poller);
            }, 3000);
        } else if (res.status === 409) {
            const data = await res.json();
            log(`Notice: ${data.message}`, 'warn');
            showToast(data.message, 'error');
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnTriggerAlbum.disabled = false;
        DOM.btnTriggerAlbum.innerText = 'RELEASE 10-TRACK ALBUM';
    }
});

async function fetchAlbumTracklist() {
    if (!DOM.albumTrackList) return;
    try {
        const res = await fetch('/api/album/tracks');
        if (res.ok) {
            const tracks = await res.json();
            if (tracks.length === 0) {
                DOM.albumTrackList.innerHTML = `<div class="track-item placeholder">No released tracks yet. Click "Release 10-Track Album" to produce.</div>`;
                return;
            }
            
            DOM.albumTrackList.innerHTML = '';
            tracks.forEach(track => {
                const item = document.createElement('div');
                item.className = 'track-item';
                item.innerHTML = `
                    <div class="track-item-name">💿 ${track.name}</div>
                    <div class="track-item-actions">
                        <button class="btn-track-play" data-url="${track.wav_url}" data-name="${track.name}">PLAY</button>
                        <a href="${track.wav_url}" download="${track.name}.wav" class="btn-track-play" style="text-decoration:none;">WAV</a>
                        <a href="${track.midi_url}" download="${track.name}.mid" class="btn-track-play" style="text-decoration:none;">MID</a>
                    </div>
                `;
                DOM.albumTrackList.appendChild(item);
            });
            
            DOM.albumTrackList.querySelectorAll('.btn-track-play[data-url]').forEach(btn => {
                btn.addEventListener('click', async (e) => {
                    const url = e.target.dataset.url;
                    const name = e.target.dataset.name;
                    log(`Streaming album track: ${name}`, 'info');
                    initWebAudio();
                    if (DOM.audio) {
                        DOM.audio.src = url;
                        DOM.audio.load();
                        DOM.audio.play();
                    }
                    if (DOM.trackTitle) DOM.trackTitle.innerText = name;
                    if (DOM.trackDetails) DOM.trackDetails.innerText = 'Full-length Produced Track • 44.1kHz WAV';
                    showToast(`Playing ${name}`, 'info');
                });
            });
        }
    } catch (e) {
        console.warn('Failed to fetch album tracks:', e);
    }
}

if (studioState.activePage === 'album') {
    fetchAlbumTracklist();
}

// --- Presets Manager ---
DOM.presetSelect?.addEventListener('change', (e) => {
    const val = e.target.value;
    log(`Applying preset: ${val}`, 'info');
    
    if (val === 'cyber_brutalist') {
        if (studioState.activePage !== 'synesthesia') window.location.href = 'index.html';
        else document.getElementById('presetCyber')?.click();
    } else if (val === 'harmonic_organic') {
        if (studioState.activePage !== 'synesthesia') window.location.href = 'index.html';
        else document.getElementById('presetOrganic')?.click();
    } else if (val === 'baroque_bach') {
        if (studioState.activePage !== 'music') window.location.href = 'music.html';
        else {
            if (DOM.musicRootSelect) DOM.musicRootSelect.value = '60';
            if (DOM.musicChordsSlider) DOM.musicChordsSlider.value = '8';
            if (DOM.musicDissonanceSlider) DOM.musicDissonanceSlider.value = '1.2';
            generateFullMusic();
        }
    } else if (val === 'cyberpunk_city') {
        if (studioState.activePage !== 'architecture') window.location.href = 'architecture.html';
        else {
            if (DOM.megaDensitySlider) DOM.megaDensitySlider.value = '40';
        }
    }
});

// --- Exports Handlers ---
DOM.exportWavBtn?.addEventListener('click', () => {
    if (!studioState.currentAudioBlob) {
        log('Downloading current synesthesia audio...', 'info');
        window.open('/release/synesthesia.wav', '_blank');
        return;
    }
    const a = document.createElement('a');
    a.href = URL.createObjectURL(studioState.currentAudioBlob);
    a.download = 'searu_synthesis.wav';
    a.click();
    log('Exported searu_synthesis.wav', 'success');
});

DOM.exportSvgBtn?.addEventListener('click', () => {
    if (!studioState.currentSvgString) {
        log('No active SVG in viewport. Generate a floorplan or truss first.', 'warn');
        showToast('No active SVG to export', 'error');
        return;
    }
    const blob = new Blob([studioState.currentSvgString], { type: 'image/svg+xml' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = `searu_blueprint.svg`;
    a.click();
    log(`Exported searu_blueprint.svg`, 'success');
    showToast('Exported SVG successfully', 'success');
});

DOM.exportMidiBtn?.addEventListener('click', () => {
    log('MIDI tracks can be downloaded directly from the Album catalog.', 'info');
    window.location.href = 'album.html';
});

function exportProfileJson() {
    const jsonStr = JSON.stringify(studioState.currentProfile, null, 2);
    const blob = new Blob([jsonStr], { type: 'application/json' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'searu_profile.json';
    a.click();
    log('Exported searu_profile.json', 'success');
    showToast('Exported searu_profile.json', 'success');
}

DOM.exportJsonBtn?.addEventListener('click', exportProfileJson);
DOM.topExportProfileBtn?.addEventListener('click', exportProfileJson);

// --- Global Keyboard Shortcuts ---
window.addEventListener('keydown', (e) => {
    if (['INPUT', 'SELECT', 'TEXTAREA'].includes(e.target.tagName)) return;

    if (e.code === 'Space') {
        e.preventDefault();
        DOM.playBtn?.click();
    } else if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        e.preventDefault();
        if (studioState.activePage === 'synesthesia') DOM.btnGenSyn?.click();
        else if (studioState.activePage === 'music') DOM.btnGenMusic?.click();
        else if (studioState.activePage === 'architecture') DOM.btnGenArch?.click();
        else if (studioState.activePage === 'album') DOM.btnTriggerAlbum?.click();
    }
});

// --- Page Ready Log ---
log(`SEARU Studio Ready. Active Workspace: ${studioState.activePage.toUpperCase()}`, 'sys');

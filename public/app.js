/**
 * ==========================================================================
 * SEARU Studio v2.5 - Core Client Application
 * Full-stack Audio DSP, Multi-Domain Annealing, SVG Pan/Zoom & Telemetry
 * ==========================================================================
 */

// --- State Management ---
const studioState = {
    activeDomain: 'music',
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
    tabs: document.querySelectorAll('.tab-btn'),
    forms: document.querySelectorAll('.domain-form'),
    inspectorTitle: document.getElementById('inspectorTitle'),
    inspectorBadge: document.getElementById('inspectorDomainBadge'),
    
    // Viewport elements
    viewportTitle: document.getElementById('viewportTitle'),
    viewportTag: document.getElementById('viewportTag'),
    svgStageWrapper: document.getElementById('svgStageWrapper'),
    svgStage: document.getElementById('svgStage'),
    audioStageWrapper: document.getElementById('audioStageWrapper'),
    canvasControls: document.getElementById('canvasControls'),
    
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
    exportJsonBtn: document.getElementById('btnExportJson'),
    topExportProfileBtn: document.getElementById('exportProfileBtn'),
    presetSelect: document.getElementById('presetSelect'),
    
    // Domain action buttons
    btnGenMusic: document.getElementById('btnGenerateMusic'),
    btnGenFm: document.getElementById('btnGenerateFm'),
    fmDissonanceSlider: document.getElementById('fmDissonanceSlider'),
    fmDissonanceVal: document.getElementById('fmDissonanceVal'),
    btnGenArch: document.getElementById('btnGenerateArch'),
    btnGenTruss: document.getElementById('btnGenerateTruss'),
    btnGenPcb: document.getElementById('btnGeneratePcb'),
    btnGenVisual: document.getElementById('btnGenerateVisual'),
    btnGenMegaCity: document.getElementById('btnGenerateMegaCity'),
    btnGenFractal: document.getElementById('btnGenerateFractal'),
    btnGenFuzz: document.getElementById('btnGenerateFuzz'),
    btnTriggerAlbum: document.getElementById('btnTriggerAlbum'),
    albumCard: document.getElementById('albumTracklistCard'),
    albumTrackList: document.getElementById('albumTrackList'),
    
    // Zoom controls
    btnZoomIn: document.getElementById('btnZoomIn'),
    btnZoomOut: document.getElementById('btnZoomOut'),
    btnResetView: document.getElementById('btnResetView')
};

// --- Logger Utility ---
function log(msg, type = 'info') {
    const line = document.createElement('div');
    line.className = `log-line ${type}`;
    const time = new Date().toLocaleTimeString();
    line.innerText = `[${time}] > ${msg}`;
    DOM.consoleBody.appendChild(line);
    DOM.consoleBody.scrollTop = DOM.consoleBody.scrollHeight;
}

DOM.clearConsoleBtn.addEventListener('click', () => {
    DOM.consoleBody.innerHTML = '';
    log('Console cleared.', 'sys');
});

// --- Web Audio & Visualizer Engine ---
let audioCtx = null;
let analyser = null;
let audioSource = null;
let isAudioInitialized = false;

function initWebAudio() {
    if (isAudioInitialized) return;
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
        log('Web Audio DSP pipeline hooked to AnalyserNode.', 'sys');
    } catch (e) {
        console.warn('Web Audio init error:', e);
    }
}

function drawSpectrum() {
    if (!analyser) return;
    const canvas = DOM.spectrumCanvas;
    const ctx = canvas.getContext('2d');
    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    
    function render() {
        requestAnimationFrame(render);
        analyser.getByteFrequencyData(dataArray);
        
        ctx.fillStyle = '#020617';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        // Draw background grid lines
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
DOM.playBtn.addEventListener('click', async () => {
    initWebAudio();
    if (audioCtx && audioCtx.state === 'suspended') {
        await audioCtx.resume();
    }
    
    if (DOM.audio.paused) {
        if (!DOM.audio.src) {
            log('No audio loaded. Generating default Bach progression...', 'info');
            generateBachAudio();
            return;
        }
        DOM.audio.play();
    } else {
        DOM.audio.pause();
    }
});

DOM.audio.addEventListener('play', () => {
    DOM.playIcon.style.display = 'none';
    DOM.pauseIcon.style.display = 'block';
});

DOM.audio.addEventListener('pause', () => {
    DOM.playIcon.style.display = 'block';
    DOM.pauseIcon.style.display = 'none';
});

DOM.audio.addEventListener('timeupdate', () => {
    if (!isNaN(DOM.audio.duration) && DOM.audio.duration > 0) {
        const perc = (DOM.audio.currentTime / DOM.audio.duration) * 100;
        DOM.seekSlider.value = perc;
        DOM.curTimeText.innerText = formatTime(DOM.audio.currentTime);
        DOM.totTimeText.innerText = formatTime(DOM.audio.duration);
    }
});

DOM.seekSlider.addEventListener('input', (e) => {
    if (!isNaN(DOM.audio.duration)) {
        DOM.audio.currentTime = (parseFloat(e.target.value) / 100) * DOM.audio.duration;
    }
});

DOM.volSlider.addEventListener('input', (e) => {
    DOM.audio.volume = parseFloat(e.target.value);
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
    DOM.audio.src = url;
    DOM.audio.load();
    DOM.audio.play().catch(e => console.log('Autoplay policy prevented audio play:', e));
    
    DOM.trackTitle.innerText = title;
    DOM.trackDetails.innerText = details;
    log(`Synthesized: ${title} (${(blob.size / 1024).toFixed(1)} KB)`, 'success');
}

// --- SVG Interactive Viewport (Pan & Zoom) ---
function updateSvgTransform() {
    DOM.svgStage.style.transform = `translate(${studioState.panX}px, ${studioState.panY}px) scale(${studioState.zoom})`;
}

DOM.btnZoomIn.addEventListener('click', () => {
    studioState.zoom = Math.min(5.0, studioState.zoom * 1.25);
    updateSvgTransform();
});

DOM.btnZoomOut.addEventListener('click', () => {
    studioState.zoom = Math.max(0.2, studioState.zoom / 1.25);
    updateSvgTransform();
});

DOM.btnResetView.addEventListener('click', () => {
    studioState.zoom = 1.0;
    studioState.panX = 0;
    studioState.panY = 0;
    updateSvgTransform();
});

DOM.svgStageWrapper.addEventListener('wheel', (e) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    studioState.zoom = Math.max(0.2, Math.min(5.0, studioState.zoom * delta));
    updateSvgTransform();
}, { passive: false });

DOM.svgStageWrapper.addEventListener('mousedown', (e) => {
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
    DOM.svgStage.innerHTML = svgString;
    
    // Reset transform
    studioState.zoom = 1.0;
    studioState.panX = 0;
    studioState.panY = 0;
    updateSvgTransform();
    
    log(`Rendered SVG: ${title}`, 'success');
}

// --- Annealing Telemetry Chart ---
function renderTelemetryChart(lossCurve = null) {
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

// --- Tab Switching Logic ---
DOM.tabs.forEach(tab => {
    tab.addEventListener('click', () => {
        const domain = tab.dataset.domain;
        switchTab(domain);
    });
});

function switchTab(domain) {
    studioState.activeDomain = domain;
    
    // Update tab styles
    DOM.tabs.forEach(t => t.classList.toggle('active', t.dataset.domain === domain));
    
    // Update forms
    DOM.forms.forEach(f => f.classList.toggle('active', f.id === `form-${domain}`));
    
    // Update Titles
    const domainNames = {
        music: { title: 'MUSIC PARAMETERS', badge: 'AUDIO DSP', mode: 'audio', tag: 'BACH HARMONICS' },
        architecture: { title: 'ARCHITECTURE PARAMETERS', badge: 'FLOORPLAN', mode: 'svg', tag: 'AABB CONSTRAINTS' },
        mechanics: { title: 'MECHANICS PARAMETERS', badge: 'STATICS', mode: 'svg', tag: '2D TRUSS FEA' },
        pcb: { title: 'PCB ROUTING PARAMETERS', badge: 'CIRCUITS', mode: 'svg', tag: 'MANHATTAN PATHS' },
        visual: { title: 'VISUAL & TYPE PARAMETERS', badge: 'GEOMETRY', mode: 'svg', tag: 'HSL ART' },
        megacity: { title: 'MEGACITY CO-EVOLUTION', badge: 'PIPELINE', mode: 'svg', tag: 'FULL BLUEPRINT' },
        fractal: { title: 'FRACTAL UNIVERSE', badge: 'RECURSIVE', mode: 'svg', tag: 'ISOMORPHIC SVG' },
        album: { title: 'ALBUM BATCH RELEASE', badge: 'RAYON MT', mode: 'audio', tag: '10-TRACK CATALOG' }
    };
    
    const info = domainNames[domain] || domainNames.music;
    DOM.inspectorTitle.innerText = info.title;
    DOM.inspectorBadge.innerText = info.badge;
    DOM.viewportTag.innerText = info.tag;
    
    if (info.mode === 'audio') {
        DOM.svgStageWrapper.style.display = 'none';
        DOM.audioStageWrapper.style.display = 'flex';
        DOM.canvasControls.style.display = 'none';
        if (domain === 'album') {
            DOM.albumCard.style.display = 'flex';
            fetchAlbumTracklist();
        } else {
            DOM.albumCard.style.display = 'none';
        }
    } else {
        DOM.svgStageWrapper.style.display = 'flex';
        DOM.audioStageWrapper.style.display = 'none';
        DOM.canvasControls.style.display = 'flex';
    }
    
    log(`Switched to workspace: ${domain.toUpperCase()}`, 'sys');
}

// --- Value Slider Badges Sync ---
function bindSlider(sliderId, badgeId, formatter) {
    const slider = document.getElementById(sliderId);
    const badge = document.getElementById(badgeId);
    if (!slider || !badge) return;
    slider.addEventListener('input', (e) => {
        badge.innerText = formatter(e.target.value);
    });
}

if (DOM.musicDissonanceSlider) {
    DOM.musicDissonanceSlider.addEventListener('input', (e) => {
        DOM.musicDissonanceVal.innerText = parseFloat(e.target.value).toFixed(1);
    });
}

if (DOM.fmDissonanceSlider) {
    DOM.fmDissonanceSlider.addEventListener('input', (e) => {
        DOM.fmDissonanceVal.innerText = parseFloat(e.target.value).toFixed(2);
    });
}

bindSlider('musicChordsSlider', 'musicChordsVal', v => `${v} Chords`);
bindSlider('musicSecSlider', 'musicSecVal', v => `${parseFloat(v).toFixed(1)}s`);
bindSlider('musicDissonanceSlider', 'musicDissonanceVal', v => parseFloat(v).toFixed(1));
bindSlider('archDensitySlider', 'archDensityVal', v => `${v} Rooms`);
bindSlider('archZoningSlider', 'archZoningVal', v => `${Math.round(v * 100)}% Commercial`);
bindSlider('archWindSlider', 'archWindVal', v => `${parseFloat(v).toFixed(1)} F`);
bindSlider('trussForceSlider', 'trussForceVal', v => `${v} N`);
bindSlider('visHueSlider', 'visHueVal', v => `${v}° Hue`);
bindSlider('visShapesSlider', 'visShapesVal', v => `${v} Shapes`);
bindSlider('megaDensitySlider', 'megaDensityVal', v => `${v} Rooms`);

// Tuning Toggle
const btnTuning12 = document.getElementById('tuning12Tet');
const btnTuningJust = document.getElementById('tuningJust');
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

// MegaColor Picker
const colorPicker = document.getElementById('megaColorPicker');
const colorVal = document.getElementById('megaColorVal');
colorPicker.addEventListener('input', (e) => {
    colorVal.innerText = e.target.value;
});

// --- API Execution Handlers ---

// 1. Music (Bach Progression)
async function generateBachAudio() {
    DOM.btnGenMusic.disabled = true;
    DOM.btnGenMusic.innerText = 'ANNEALING VOICES...';
    log('Igniting The Crucible: Annealing 4-voice SATB counterpoint...', 'info');
    
    const root = parseFloat(document.getElementById('musicRootSelect').value);
    const num_chords = parseInt(document.getElementById('musicChordsSlider').value);
    const seconds_per_chord = parseFloat(document.getElementById('musicSecSlider').value);
    
    try {
        const res = await fetch('/api/music/bach', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ root_note: root, num_chords, seconds_per_chord })
        });
        
        if (res.ok) {
            const blob = await res.blob();
            loadAndPlayAudioBlob(blob, `Bach Progression (${num_chords} Chords)`, `Root MIDI: ${root} • SATB Polyphony`);
            renderTelemetryChart();
            DOM.statLoss.innerText = '0.042';
            DOM.statTemp.innerText = '0.001°';
        } else {
            log('Error generating Bach audio.', 'err');
        }
    } catch (e) {
        log(`Network error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenMusic.disabled = false;
        DOM.btnGenMusic.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg> ANNEAL & SYNTHESIZE BACH`;
    }
}
DOM.btnGenMusic.addEventListener('click', generateBachAudio);

async function generateFmAudio() {
    DOM.btnGenFm.disabled = true;
    DOM.btnGenFm.innerText = 'SYNTHESIZING...';
    log('Igniting The Crucible: Annealing FM parameters to target dissonance...', 'info');
    
    const diss = parseFloat(DOM.fmDissonanceSlider.value);
    
    try {
        const res = await fetch(`/api/music/fm?dissonance=${diss}`, { method: 'GET' });
        if (res.ok) {
            const blob = await res.blob();
            loadAndPlayAudioBlob(blob, `Dialectical FM Timbre`, `Target Dissonance: ${diss.toFixed(2)}`);
            renderTelemetryChart();
        } else {
            log('Error generating FM audio.', 'err');
        }
    } catch (e) {
        log(`Network error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenFm.disabled = false;
        DOM.btnGenFm.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 12h-4l-3 9L9 3l-3 9H2"></path></svg> SYNTHESIZE FM TIMBRE`;
    }
}
DOM.btnGenFm.addEventListener('click', generateFmAudio);

// 2. Architecture (Floorplan)
DOM.btnGenArch.addEventListener('click', async () => {
    DOM.btnGenArch.disabled = true;
    DOM.btnGenArch.innerText = 'EVOLVING SPATIAL BLUEPRINT...';
    log('Running Dual-Chaos Min-Max Room Evolution against Wind Force...', 'info');
    
    const density = parseInt(document.getElementById('archDensitySlider').value);
    const zoning_ratio = parseFloat(document.getElementById('archZoningSlider').value);
    const max_wind_force = parseFloat(document.getElementById('archWindSlider').value);
    
    try {
        const res = await fetch('/api/architecture/floorplan', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ density, zoning_ratio, max_wind_force })
        });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, `Architecture Layout (${density} Rooms)`);
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenArch.disabled = false;
        DOM.btnGenArch.innerText = 'EVOLVE FLOORPLAN';
    }
});

// 3. Mechanics (Truss)
DOM.btnGenTruss.addEventListener('click', async () => {
    DOM.btnGenTruss.disabled = true;
    DOM.btnGenTruss.innerText = 'SOLVING TOPOLOGY...';
    log('Optimizing 2D Truss load paths and minimizing mass...', 'info');
    
    try {
        const res = await fetch('/api/mechanics/truss', { method: 'POST' });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, '2D Truss Topology Statics');
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenTruss.disabled = false;
        DOM.btnGenTruss.innerText = 'OPTIMIZE TRUSS TOPOLOGY';
    }
});

// 4. PCB Routing
DOM.btnGenPcb.addEventListener('click', async () => {
    DOM.btnGenPcb.disabled = true;
    DOM.btnGenPcb.innerText = 'ROUTING NETS...';
    log('Annealing Manhattan waypoints for 0-collision crossing nets...', 'info');
    
    try {
        const res = await fetch('/api/pcb_routing/route', { method: 'POST' });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, 'PCB 2-Net Routing');
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenPcb.disabled = false;
        DOM.btnGenPcb.innerText = 'SOLVE PCB ROUTING';
    }
});

// 5. Visual Art & Typography
const visModeArt = document.getElementById('visModeArt');
const visModeType = document.getElementById('visModeType');
const visArtControls = document.getElementById('visArtControls');

visModeArt.addEventListener('click', () => {
    visModeArt.classList.add('active');
    visModeType.classList.remove('active');
    visArtControls.style.display = 'block';
});
visModeType.addEventListener('click', () => {
    visModeType.classList.add('active');
    visModeArt.classList.remove('active');
    visArtControls.style.display = 'none';
});

DOM.btnGenVisual.addEventListener('click', async () => {
    DOM.btnGenVisual.disabled = true;
    DOM.btnGenVisual.innerText = 'COMPOSING...';
    
    if (visModeArt.classList.contains('active')) {
        log('Generating HSL Golden Ratio geometric artwork...', 'info');
        const num_shapes = parseInt(document.getElementById('visShapesSlider').value);
        const base_hue = parseFloat(document.getElementById('visHueSlider').value);
        try {
            const res = await fetch('/api/visual/art', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ num_shapes, base_hue, fractal_depth: 8 })
            });
            if (res.ok) {
                const svg = await res.text();
                displaySvg(svg, `Visual Geometric Art (${num_shapes} Shapes)`);
            }
        } catch (e) {
            log(`Error: ${e.message}`, 'err');
        }
    } else {
        log('Annealing Bezier curvature for typography glyph...', 'info');
        try {
            const res = await fetch('/api/typography/glyph', { method: 'POST' });
            if (res.ok) {
                const svg = await res.text();
                displaySvg(svg, 'Bezier Typography Glyph');
            }
        } catch (e) {
            log(`Error: ${e.message}`, 'err');
        }
    }
    DOM.btnGenVisual.disabled = false;
    DOM.btnGenVisual.innerText = 'RENDER ARTWORK';
});

// 6. MegaCity Pipeline
DOM.btnGenMegaCity.addEventListener('click', async () => {
    DOM.btnGenMegaCity.disabled = true;
    DOM.btnGenMegaCity.innerText = 'CO-EVOLVING MEGACITY...';
    log('Co-evolving Architecture + Truss + Inverse PBR Materials...', 'info');
    
    const hex = colorPicker.value;
    const r = parseInt(hex.slice(1, 3), 16) / 255;
    const g = parseInt(hex.slice(3, 5), 16) / 255;
    const b = parseInt(hex.slice(5, 7), 16) / 255;
    const density = parseInt(document.getElementById('megaDensitySlider').value);
    
    const profile = {
        arch: { density, zoning_ratio: 0.5, max_wind_force: 50.0 },
        visual: { fractal_depth: 12, base_hue: 200.0 },
        mechanics: { target_r: r, target_g: g, target_b: b }
    };
    
    try {
        const res = await fetch('/api/megacity/pipeline', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(profile)
        });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, 'MegaCity Co-Evolution Blueprint');
            renderTelemetryChart();
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenMegaCity.disabled = false;
        DOM.btnGenMegaCity.innerText = 'GENERATE MEGACITY PIPELINE';
    }
});

// 7. Fractal Universe
DOM.btnGenFractal.addEventListener('click', async () => {
    DOM.btnGenFractal.disabled = true;
    DOM.btnGenFractal.innerText = 'RECURSIVELY PROJECTING...';
    log('Expanding Isomorphic Multi-Depth Fractal Universe (Depth 0 -> 3)...', 'info');
    
    try {
        const res = await fetch('/api/fractal/universe', { method: 'POST' });
        if (res.ok) {
            const svg = await res.text();
            displaySvg(svg, 'Fractal Universe Multiverse');
        }
    } catch (e) {
        log(`Network error: ${e.message}`, 'err');
    } finally {
        DOM.btnGenFractal.disabled = false;
        DOM.btnGenFractal.innerText = 'EXPAND FRACTAL UNIVERSE';
    }
});

// 9. Multi-Domain Fuzzing
if (DOM.btnGenFuzz) {
    DOM.btnGenFuzz.addEventListener('click', async () => {
        DOM.btnGenFuzz.disabled = true;
        DOM.btnGenFuzz.innerText = 'IGNITING MULTI-DOMAIN FUZZING...';
        log('Initializing Navier-Stokes Fluid + Kinematics + Silicon Photonics...', 'info');
        log('Applying Heat vs Viscosity / Vibration vs Optical Radius cross-penalties.', 'sys');

        try {
            const res = await fetch('/api/science/multidomain_fuzz', {
                method: 'POST',
            });
            if (res.ok) {
                const data = await res.json();
                log(`Fuzzing Optimization Complete. Best Score: ${data.final_score}`, 'success');
                log(`Final Genes [9D]:`, 'info');
                log(`  Fluid (Vorticity, Pressure, Viscosity, Strain): ${data.genes.slice(0,4).map(v => v.toFixed(3)).join(', ')}`, 'sys');
                log(`  Kinematics (Stiffness, Damping, Freq): ${data.genes.slice(4,7).map(v => v.toFixed(3)).join(', ')}`, 'sys');
                log(`  Photonics (Radius, Power): ${data.genes.slice(7,9).map(v => v.toFixed(3)).join(', ')}`, 'sys');
                renderTelemetryChart();
            } else {
                log('Error optimizing multi-domain fuzzing.', 'err');
            }
        } catch (e) {
            log(`Network error: ${e.message}`, 'err');
        } finally {
            DOM.btnGenFuzz.disabled = false;
            DOM.btnGenFuzz.innerText = 'IGNITE MULTI-DOMAIN FUZZING';
        }
    });
}


// 8. Album Release & Catalog
DOM.btnTriggerAlbum.addEventListener('click', async () => {
    DOM.btnTriggerAlbum.disabled = true;
    DOM.btnTriggerAlbum.innerText = 'SPAWNING RAYON WORKERS...';
    log('Initiating Rayon Parallel Album Pipeline (10 tracks)...', 'info');
    
    try {
        const res = await fetch('/api/album/release', { method: 'POST' });
        if (res.ok) {
            const data = await res.json();
            log(`Success: ${data.status}`, 'success');
            // Poll for tracks every 3 seconds
            let polls = 0;
            const poller = setInterval(async () => {
                polls++;
                await fetchAlbumTracklist();
                if (polls > 8) clearInterval(poller);
            }, 3000);
        }
    } catch (e) {
        log(`Error: ${e.message}`, 'err');
    } finally {
        DOM.btnTriggerAlbum.disabled = false;
        DOM.btnTriggerAlbum.innerText = 'RELEASE 10-TRACK ALBUM';
    }
});

async function fetchAlbumTracklist() {
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
                    DOM.audio.src = url;
                    DOM.audio.load();
                    DOM.audio.play();
                    DOM.trackTitle.innerText = name;
                    DOM.trackDetails.innerText = 'Full-length Produced Track • 44.1kHz WAV';
                });
            });
        }
    } catch (e) {
        console.warn('Failed to fetch album tracks:', e);
    }
}

// --- Presets Manager ---
DOM.presetSelect.addEventListener('change', (e) => {
    const val = e.target.value;
    log(`Applying preset: ${val}`, 'info');
    
    if (val === 'baroque_bach') {
        document.getElementById('musicRootSelect').value = '60'; // C4
        document.getElementById('musicChordsSlider').value = '8';
        document.getElementById('musicChordsVal').innerText = '8 Chords';
        document.getElementById('musicDissonanceSlider').value = '1.2';
        document.getElementById('musicDissonanceVal').innerText = '1.2';
        switchTab('music');
        generateBachAudio();
    } else if (val === 'cyberpunk_city') {
        document.getElementById('megaDensitySlider').value = '40';
        document.getElementById('megaDensityVal').innerText = '40 Rooms';
        colorPicker.value = '#f43f5e';
        colorVal.innerText = '#f43f5e';
        switchTab('megacity');
    } else if (val === 'drone_truss') {
        document.getElementById('trussForceSlider').value = '3500';
        document.getElementById('trussForceVal').innerText = '3500 N';
        switchTab('mechanics');
    } else if (val === 'high_density_pcb') {
        switchTab('pcb');
    } else if (val === 'acoustic_just') {
        btnTuningJust.click();
        document.getElementById('musicRootSelect').value = '69'; // A4
        switchTab('music');
    }
});

// --- Exports Handlers ---
DOM.exportWavBtn.addEventListener('click', () => {
    if (!studioState.currentAudioBlob) {
        log('No active audio to export. Generate a track first.', 'warn');
        return;
    }
    const a = document.createElement('a');
    a.href = URL.createObjectURL(studioState.currentAudioBlob);
    a.download = 'searu_synthesis.wav';
    a.click();
    log('Exported searu_synthesis.wav', 'success');
});

DOM.exportSvgBtn.addEventListener('click', () => {
    if (!studioState.currentSvgString) {
        log('No active SVG to export. Generate a visual first.', 'warn');
        return;
    }
    const blob = new Blob([studioState.currentSvgString], { type: 'image/svg+xml' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = `searu_${studioState.activeDomain}.svg`;
    a.click();
    log(`Exported searu_${studioState.activeDomain}.svg`, 'success');
});

DOM.exportMidiBtn.addEventListener('click', () => {
    log('To download MIDI, release an album track or click MID in Album catalog.', 'info');
    switchTab('album');
});

function exportProfileJson() {
    const jsonStr = JSON.stringify(studioState.currentProfile, null, 2);
    const blob = new Blob([jsonStr], { type: 'application/json' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'searu_profile.json';
    a.click();
    log('Exported searu_profile.json', 'success');
}

DOM.exportJsonBtn.addEventListener('click', exportProfileJson);
DOM.topExportProfileBtn.addEventListener('click', exportProfileJson);

// --- Initialization ---
log('SEARU Studio Ready. Initializing with default Music domain.', 'sys');

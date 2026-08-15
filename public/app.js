// Default Profile State
const profile = {
    culture: {
        tuning: "12-TET", // or "Just Intonation"
        phrase_length_bars: 4,
        rhythmic_grid: "4/4", // or "Polyrhythm"
    },
    physics: {
        dissonance_tolerance: 2.0, // 0.0 to 10.0
        fractal_chaos: 5.0 // 1.0 to 10.0
    }
};

// DOM Elements
const phraseInput = document.getElementById('phraseLength');
const phraseVal = document.getElementById('phraseLengthVal');
const tuningInput = document.getElementById('tuningScale');
const gridInput = document.getElementById('rhythmicGrid');
const consoleOutput = document.getElementById('consoleOutput');
const canvas = document.getElementById('tensionCanvas');
const ctx = canvas.getContext('2d');

function logToConsole(msg) {
    const p = document.createElement('p');
    p.className = 'update';
    p.innerText = `> ${msg}`;
    consoleOutput.appendChild(p);
    consoleOutput.scrollTop = consoleOutput.scrollHeight;
}

// Phrase Length
phraseInput.addEventListener('input', (e) => {
    profile.culture.phrase_length_bars = parseInt(e.target.value);
    phraseVal.innerText = `${profile.culture.phrase_length_bars} Bars`;
    logToConsole(`Set Phrase Structure to ${profile.culture.phrase_length_bars} bars.`);
    drawCanvas();
});

// Tuning
tuningInput.addEventListener('change', (e) => {
    profile.culture.tuning = e.target.value == "0" ? "12-TET" : "Just Intonation";
    logToConsole(`Tuning system set to ${profile.culture.tuning}.`);
});

// Grid
gridInput.addEventListener('change', (e) => {
    profile.culture.rhythmic_grid = e.target.value == "0" ? "4/4" : "Polyrhythm";
    logToConsole(`Rhythmic Grid set to ${profile.culture.rhythmic_grid}.`);
});

// Dial Logic
function setupDial(knobId, indId, valId, profileKey, min, max, initial) {
    const knob = document.getElementById(knobId);
    const ind = document.getElementById(indId);
    const valDisp = document.getElementById(valId);
    let val = initial;
    
    // Map value to -135deg to +135deg
    const updateDial = (v) => {
        val = Math.max(min, Math.min(max, v));
        const perc = (val - min) / (max - min);
        const deg = -135 + (perc * 270);
        ind.style.transform = `rotate(${deg}deg)`;
        valDisp.innerText = val.toFixed(1);
        profile.physics[profileKey] = parseFloat(val.toFixed(1));
        drawCanvas();
    };

    updateDial(val);

    let isDragging = false;
    let startY = 0;
    let startVal = val;

    knob.addEventListener('mousedown', (e) => {
        isDragging = true;
        startY = e.clientY;
        startVal = val;
    });

    window.addEventListener('mousemove', (e) => {
        if (!isDragging) return;
        const deltaY = startY - e.clientY; // drag up increases
        const deltaVal = (deltaY / 100) * (max - min); 
        updateDial(startVal + deltaVal);
    });

    window.addEventListener('mouseup', () => {
        if (isDragging) {
            isDragging = false;
            logToConsole(`Updated ${profileKey} to ${val.toFixed(1)}`);
        }
    });
}

setupDial('tensionKnob', 'tensionInd', 'tensionVal', 'dissonance_tolerance', 0.0, 10.0, 2.0);
setupDial('fractalKnob', 'fractalInd', 'fractalVal', 'fractal_chaos', 1.0, 10.0, 5.0);

// Canvas Visualizer
function drawCanvas() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const w = canvas.width;
    const h = canvas.height;
    
    // Draw grid
    ctx.strokeStyle = '#1f2833';
    ctx.lineWidth = 1;
    for(let i=0; i<w; i+=40) {
        ctx.beginPath(); ctx.moveTo(i, 0); ctx.lineTo(i, h); ctx.stroke();
    }
    
    // Draw Tension Curve based on parameters
    ctx.beginPath();
    ctx.moveTo(0, h - 20);
    
    const bars = profile.culture.phrase_length_bars;
    const dissonance = profile.physics.dissonance_tolerance;
    const chaos = profile.physics.fractal_chaos;
    
    for(let i = 0; i <= w; i++) {
        let progress = i / w;
        
        // Base sine wave representing macro energy
        let base = Math.sin(progress * Math.PI) * 0.5;
        
        // Add tension peaks depending on dissonance
        let tensionPeak = Math.pow(Math.sin(progress * Math.PI * bars), 2) * (dissonance / 10.0) * 0.3;
        
        // Add fractal noise
        let noise = (Math.random() * 2 - 1) * (chaos / 10.0) * 0.1;
        
        let y = h - 20 - ((base + tensionPeak + noise) * h * 0.8);
        ctx.lineTo(i, Math.max(10, Math.min(h-10, y)));
    }
    
    ctx.strokeStyle = '#66fcf1';
    ctx.lineWidth = 2;
    ctx.stroke();
    
    // Fill under curve
    ctx.lineTo(w, h);
    ctx.lineTo(0, h);
    ctx.fillStyle = 'rgba(102, 252, 241, 0.1)';
    ctx.fill();
}

drawCanvas();

// Export Function
document.getElementById('exportBtn').addEventListener('click', () => {
    const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(profile, null, 2));
    const a = document.createElement('a');
    a.href = dataStr;
    a.download = "searu_profile.json";
    document.body.appendChild(a);
    a.click();
    a.remove();
    logToConsole("Exported searu_profile.json successfully.");
});

// Music Generation
const genMusicBtn = document.getElementById('genMusicBtn');
const musicPlayer = document.getElementById('musicPlayer');
const barsSlider = document.getElementById('barsSlider');
const barsVal = document.getElementById('barsVal');
const bpmSlider = document.getElementById('bpmSlider');
const bpmVal = document.getElementById('bpmVal');

if (barsSlider) {
    barsSlider.addEventListener('input', (e) => {
        barsVal.innerText = `${e.target.value} Bars`;
    });
}
if (bpmSlider) {
    bpmSlider.addEventListener('input', (e) => {
        bpmVal.innerText = `${e.target.value} BPM`;
    });
}

if (genMusicBtn) {
    genMusicBtn.addEventListener('click', async () => {
        genMusicBtn.innerText = "GENERATING (Annealing)...";
        genMusicBtn.disabled = true;
        logToConsole(`Composing track with Dissonance: ${profile.physics.dissonance_tolerance}, Chaos: ${profile.physics.fractal_chaos}`);
        
        try {
            // Also update the UI bars and BPM if they were manipulated
            const barsSlider = document.getElementById('barsSlider');
            if (barsSlider) profile.culture.phrase_length_bars = parseInt(barsSlider.value);

            // Initialize Telemetry UI
            const telemetryBox = document.getElementById('telemetryBox');
            const telemetryStatus = document.getElementById('telemetryStatus');
            const telemetryCanvas = document.getElementById('telemetryCanvas');
            let tctx = null;
            let history = [];
            let evtSource = null;

            if (telemetryBox && telemetryCanvas) {
                telemetryBox.style.display = 'block';
                tctx = telemetryCanvas.getContext('2d');
                tctx.clearRect(0, 0, telemetryCanvas.width, telemetryCanvas.height);
                
                evtSource = new EventSource('/api/telemetry');
                evtSource.onmessage = (e) => {
                    const data = JSON.parse(e.data);
                    history.push({ fitness: data.fitness, is_epiphany: data.is_epiphany });
                    if (history.length > 300) history.shift();
                    
                    telemetryStatus.innerText = `Temp: ${data.temp.toFixed(4)} | Fitness: ${data.fitness.toFixed(2)}`;
                    
                    if (data.is_epiphany) {
                        logToConsole(`✨ [Aesthetic Epiphany] Forced mutation at iteration ${data.iteration}!`);
                    }

                    // Draw chart
                    tctx.clearRect(0, 0, telemetryCanvas.width, telemetryCanvas.height);
                    tctx.beginPath();
                    
                    const maxFit = Math.max(...history.map(h => h.fitness), 0.1);
                    const minFit = Math.min(...history.map(h => h.fitness), 0.0);
                    const range = (maxFit - minFit) || 1.0;
                    
                    history.forEach((point, i) => {
                        const x = (i / history.length) * telemetryCanvas.width;
                        const y = telemetryCanvas.height - ((point.fitness - minFit) / range * telemetryCanvas.height);
                        if (i === 0) tctx.moveTo(x, y);
                        else tctx.lineTo(x, y);
                    });
                    tctx.strokeStyle = '#f59e0b';
                    tctx.lineWidth = 2;
                    tctx.stroke();

                    // Draw Epiphany Highlights
                    history.forEach((point, i) => {
                        if (point.is_epiphany) {
                            const x = (i / history.length) * telemetryCanvas.width;
                            const y = telemetryCanvas.height - ((point.fitness - minFit) / range * telemetryCanvas.height);
                            tctx.beginPath();
                            tctx.arc(x, y, 4, 0, 2 * Math.PI);
                            tctx.fillStyle = '#b829ea'; // Neon Purple
                            tctx.fill();
                            tctx.strokeStyle = '#fff';
                            tctx.lineWidth = 1;
                            tctx.stroke();
                        }
                    });
                };
            }

            const response = await fetch('/api/music/generate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(profile)
            });
            
            if (response.ok) {
                const blob = await response.blob();
                const audioUrl = URL.createObjectURL(blob);
                musicPlayer.src = audioUrl;
                musicPlayer.style.display = 'block';
                musicPlayer.play();
                logToConsole("Music generated successfully. Playing preview...");
            } else {
                logToConsole("Error generating music.");
            }
        } catch (e) {
            logToConsole("Error: " + e.message);
        } finally {
            if (typeof evtSource !== 'undefined' && evtSource) {
                evtSource.close();
            }
            genMusicBtn.innerText = "GENERATE & PLAY MUSIC";
            genMusicBtn.disabled = false;
        }
    });
}

// Urban Planning Logic
const densitySlider = document.getElementById('densitySlider');
const densityVal = document.getElementById('densityVal');
const zoningSlider = document.getElementById('zoningSlider');
const zoningVal = document.getElementById('zoningVal');
const windSlider = document.getElementById('windSlider');
const windVal = document.getElementById('windVal');

const fractalSlider = document.getElementById('fractalSlider');
const fractalVal = document.getElementById('fractalVal');
const hueSlider = document.getElementById('hueSlider');
const hueVal = document.getElementById('hueVal');

const colorRSlider = document.getElementById('colorRSlider');
const colorRVal = document.getElementById('colorRVal');
const colorGSlider = document.getElementById('colorGSlider');
const colorGVal = document.getElementById('colorGVal');
const colorBSlider = document.getElementById('colorBSlider');
const colorBVal = document.getElementById('colorBVal');

const genCityBtn = document.getElementById('genCityBtn');
const citySvgContainer = document.getElementById('citySvgContainer');

const megaProfile = {
    arch: {
        density: 20,
        zoning_ratio: 0.5,
        max_wind_force: 50.0
    },
    visual: {
        fractal_depth: 12,
        base_hue: 200.0
    },
    mechanics: {
        target_r: 0.2,
        target_g: 0.2,
        target_b: 0.2
    }
};

densitySlider.addEventListener('input', (e) => {
    megaProfile.arch.density = parseInt(e.target.value);
    densityVal.innerText = `${megaProfile.arch.density} Rooms`;
});

zoningSlider.addEventListener('input', (e) => {
    megaProfile.arch.zoning_ratio = parseFloat(e.target.value);
    zoningVal.innerText = `${Math.round(megaProfile.arch.zoning_ratio * 100)}% Com`;
});

windSlider.addEventListener('input', (e) => {
    megaProfile.arch.max_wind_force = parseFloat(e.target.value);
    windVal.innerText = `${megaProfile.arch.max_wind_force.toFixed(1)} F`;
});

fractalSlider.addEventListener('input', (e) => {
    megaProfile.visual.fractal_depth = parseInt(e.target.value);
    fractalVal.innerText = `Depth: ${megaProfile.visual.fractal_depth}`;
});

hueSlider.addEventListener('input', (e) => {
    megaProfile.visual.base_hue = parseFloat(e.target.value);
    hueVal.innerText = `${megaProfile.visual.base_hue}°`;
});

colorRSlider.addEventListener('input', (e) => {
    megaProfile.mechanics.target_r = parseFloat(e.target.value);
    colorRVal.innerText = `${megaProfile.mechanics.target_r.toFixed(1)}`;
});
colorGSlider.addEventListener('input', (e) => {
    megaProfile.mechanics.target_g = parseFloat(e.target.value);
    colorGVal.innerText = `${megaProfile.mechanics.target_g.toFixed(1)}`;
});
colorBSlider.addEventListener('input', (e) => {
    megaProfile.mechanics.target_b = parseFloat(e.target.value);
    colorBVal.innerText = `${megaProfile.mechanics.target_b.toFixed(1)}`;
});

genCityBtn.addEventListener('click', async () => {
    genCityBtn.innerText = "GENERATING (Co-Evolving)...";
    genCityBtn.disabled = true;
    logToConsole(`Evolving MegaCity Pipeline...`);
    
    try {
        const response = await fetch('/api/megacity/pipeline', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(megaProfile)
        });
        
        if (response.ok) {
            const svgText = await response.text();
            citySvgContainer.innerHTML = svgText;
            
            // Make SVG responsive
            const svgEl = citySvgContainer.querySelector('svg');
            if(svgEl) {
                svgEl.style.width = '100%';
                svgEl.style.height = '100%';
            }
            logToConsole("MegaCity generated successfully. Blueprint exported to public/blender/");
            document.getElementById('blenderInfo').style.display = 'block';
        } else {
            logToConsole("Error generating MegaCity.");
        }
    } catch (err) {
        console.error(err);
        logToConsole("Network error.");
    } finally {
        genCityBtn.innerText = "GENERATE MEGACITY";
        genCityBtn.disabled = false;
    }
});


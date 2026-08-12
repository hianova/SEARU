async function handleRequest(btn, url, type, containerSelector) {
    const card = btn.closest('.card');
    const resultArea = card.querySelector('.result-area');
    const container = resultArea.querySelector(containerSelector);
    const loader = resultArea.querySelector('.loader');

    // Reset state
    card.classList.add('loading');
    container.classList.add('hidden');
    btn.disabled = true;

    try {
        const response = await fetch(url);
        if (!response.ok) throw new Error('Network error');

        if (type === 'audio') {
            const blob = await response.blob();
            const audioUrl = URL.createObjectURL(blob);
            container.src = audioUrl;
            container.classList.remove('hidden');
            container.play().catch(e => console.log("Auto-play prevented", e));
        } 
        else if (type === 'svg') {
            const svgText = await response.text();
            container.innerHTML = svgText;
            container.classList.remove('hidden');
        }
        else if (type === 'json') {
            const data = await response.json();
            container.textContent = JSON.stringify(data, null, 2);
            container.classList.remove('hidden');
        }
    } catch (err) {
        console.error(err);
        container.innerHTML = `<span style="color: #ef4444;">Error generating content.</span>`;
        if (type !== 'audio') container.classList.remove('hidden');
    } finally {
        card.classList.remove('loading');
        btn.disabled = false;
    }
}

function generateFractal(btn) {
    handleRequest(btn, '/api/fractal/universe', 'svg', '.svg-container').then(() => {
        setupInfiniteZoom();
    });
}

function generateMegaCity(btn) {
    handleRequest(btn, '/api/megacity/pipeline', 'svg', '.svg-container');
}

function generateMusic(btn) {
    handleRequest(btn, '/api/music/bach', 'audio', 'audio');
}

function generateVisual(btn) {
    handleRequest(btn, '/api/visual/art', 'svg', '.svg-container');
}

function generateMechanics(btn) {
    handleRequest(btn, '/api/mechanics/truss', 'svg', '.svg-container');
}

function generateMaterials(btn) {
    handleRequest(btn, '/api/materials/match', 'json', '.json-container');
}

function generateArch(btn) {
    handleRequest(btn, '/api/architecture/floorplan', 'svg', '.svg-container');
}

function generateUi(btn) {
    handleRequest(btn, '/api/ui_layout/optimize', 'json', '.json-container');
}

function generatePcb(btn) {
    handleRequest(btn, '/api/pcb_routing/route', 'svg', '.svg-container');
}

function generateTypo(btn) {
    handleRequest(btn, '/api/typography/glyph', 'svg', '.svg-container');
}

function generateAnim(btn) {
    handleRequest(btn, '/api/procedural_animation/curve', 'json', '.json-container');
}

async function runAlbumRelease() {
    try {
        const res = await fetch('/api/album/release');
        const data = await res.json();
        alert(data.status);
    } catch (e) {
        alert('Error: ' + e);
    }
}

// Infinite Zoom / Pan Logic
function setupInfiniteZoom() {
    const container = document.getElementById('fractal-container');
    const svg = document.getElementById('fractal-svg');
    if (!svg) return;

    let scale = 1;
    let panX = 0;
    let panY = 0;
    let isDragging = false;
    let startX = 0;
    let startY = 0;

    // Apply transform
    const updateTransform = () => {
        svg.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
        svg.style.transformOrigin = 'center';
    };

    // Zoom on wheel
    container.addEventListener('wheel', (e) => {
        e.preventDefault();
        const zoomIntensity = 0.1;
        const wheel = e.deltaY < 0 ? 1 : -1;
        scale *= Math.exp(wheel * zoomIntensity);
        updateTransform();
    }, { passive: false });

    // Pan on drag
    container.addEventListener('mousedown', (e) => {
        isDragging = true;
        startX = e.clientX - panX;
        startY = e.clientY - panY;
        container.style.cursor = 'grabbing';
    });

    window.addEventListener('mousemove', (e) => {
        if (!isDragging) return;
        panX = e.clientX - startX;
        panY = e.clientY - startY;
        updateTransform();
    });

    window.addEventListener('mouseup', () => {
        isDragging = false;
        container.style.cursor = 'grab';
    });
    
    updateTransform();
}


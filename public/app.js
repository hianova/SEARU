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

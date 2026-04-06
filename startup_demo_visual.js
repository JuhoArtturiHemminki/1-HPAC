(function() {
    const oldOverlay = document.getElementById('hso-overlay');
    if (oldOverlay) oldOverlay.remove();

    const container = document.createElement('div');
    container.id = 'hso-overlay';
    Object.assign(container.style, {
        position: 'fixed', top: '0', left: '0', width: '100vw', height: '100vh',
        background: 'radial-gradient(circle, #050a10 0%, #000 100%)',
        zIndex: '10001', display: 'flex', flexDirection: 'column',
        alignItems: 'center', justifyContent: 'center', overflow: 'hidden',
        color: '#00ffff', fontFamily: '"Segoe UI", Roboto, monospace'
    });

    const canvas = document.createElement('canvas');
    canvas.width = 600; canvas.height = 600;
    container.appendChild(canvas);

    const ui = document.createElement('div');
    ui.innerHTML = `
        <div style="text-align:center; margin-top:20px;">
            <h1 style="margin:0; letter-spacing:10px; font-weight:100; color:#fff; text-shadow:0 0 20px #00ffff;">1-HPAC</h1>
            <p style="color:#00ff41; font-size:0.8rem;">HEMMINKI SPECTRAL ONTOLOGY • LATTICE SYNC ACTIVE</p>
            <div id="stats" style="display:flex; gap:40px; margin-top:20px; font-size:1.5rem; font-weight:bold;">
                <div>TEMP: <span id="v-temp">24.5</span>°C</div>
                <div>POWER: <span id="v-power" style="color:#00ff00">+0.0</span>W</div>
            </div>
            <div id="status-msg" style="margin-top:20px; font-style:italic; color:#555;">Initializing PHI-lock...</div>
        </div>
    `;
    container.appendChild(ui);
    document.body.appendChild(container);

    const ctx = canvas.getContext('2d');
    const PHI = 1.618033988749895;
    const H_C = 5.0832104;
   
    let temp = 24.5;
    let drift = 0.5;
    let particles = [];
    for(let i=0; i<300; i++) particles.push({ x: Math.random()*600, y: Math.random()*600, vx: 0, vy: 0 });

    function draw() {
        let entropy = Math.random();
        drift = (drift + (entropy % PHI) / H_C) / PHI;
        temp -= (drift * 0.05);
        if (temp < 18) temp += 0.02;

        ctx.fillStyle = 'rgba(0, 0, 0, 0.15)';
        ctx.fillRect(0, 0, 600, 600);

        const hue = Math.max(180, 180 + (temp - 18) * 15);
        const glow = Math.max(5, 25 - (temp - 18) * 2);

        particles.forEach((p) => {
            let targetX = Math.round(p.x / 40) * 40;
            let targetY = Math.round(p.y / 40) * 40;
           
            p.vx += (targetX - p.x) * (1 - drift) * 0.1;
            p.vy += (targetY - p.y) * (1 - drift) * 0.1;
           
            p.vx += (Math.random() - 0.5) * (temp - 17);
            p.vy += (Math.random() - 0.5) * (temp - 17);

            p.x += p.vx; p.y += p.vy;
            p.vx *= 0.8; p.vy *= 0.8;

            ctx.beginPath();
            ctx.arc(p.x, p.y, 1.5, 0, Math.PI*2);
            ctx.fillStyle = `hsla(${hue}, 100%, 50%, 0.8)`;
            ctx.shadowBlur = glow;
            ctx.shadowColor = `hsl(${hue}, 100%, 50%)`;
            ctx.fill();
        });

        document.getElementById('v-temp').innerText = temp.toFixed(1);
        document.getElementById('v-power').innerText = "+" + Math.max(0, (24.5 - temp) * 25).toFixed(0);
       
        const status = document.getElementById('status-msg');
        if (temp < 19) {
            status.innerText = "!!! GHOST-PHASE DETECTED: SILICON TRANSPARENCY 42% !!!";
            status.style.color = "#ff00ff";
            container.style.boxShadow = "inset 0 0 100px #ff00ff";
        } else if (temp < 21) {
            status.innerText = "HSO RESONANCE ACHIEVED. ENERGY REVERSAL COMMENCING.";
            status.style.color = "#00ffff";
        }

        requestAnimationFrame(draw);
    }

    draw();
})();

import init, { GenerativeArt } from '../pkg/gagl_wasm.js';

async function run() {
    await init();
    
    const art = new GenerativeArt(400, 400);
    
    const redSlider = document.getElementById('red');
    const greenSlider = document.getElementById('green');
    const blueSlider = document.getElementById('blue');
    
    const redValue = document.getElementById('red-value');
    const greenValue = document.getElementById('green-value');
    const blueValue = document.getElementById('blue-value');
    
    function updateColor() {
        const r = parseFloat(redSlider.value);
        const g = parseFloat(greenSlider.value);
        const b = parseFloat(blueSlider.value);
        
        redValue.textContent = r.toFixed(2);
        greenValue.textContent = g.toFixed(2);
        blueValue.textContent = b.toFixed(2);
        
        art.set_color(r, g, b);
    }
    
    redSlider.addEventListener('input', updateColor);
    greenSlider.addEventListener('input', updateColor);
    blueSlider.addEventListener('input', updateColor);
    
    updateColor();
    
    function animate() {
        art.draw();
        requestAnimationFrame(animate);
    }
    
    animate();
}

run();
import init, { JellyFish, Nudibranch } from '../pkg/gagl_wasm.js';

var animation_number = null;
var art = null;
const arts = {
    "Jellyfish": {
        "class": JellyFish,
        "ref": "https://x.com/yuruyurau/status/1865420201086636376"
    },
    "Nudibranch": {
        "class": Nudibranch, 
        "ref": "https://x.com/yuruyurau/status/1866142306119885279"
    }
}

async function run(title) {
    destroyAndRecreateCanvas('canvas_container','canvas');
    let obj = arts[title]['class'];

    if (arts[title]['ref']){
        document.getElementById('ref').setAttribute('href', arts[title]['ref']);
        document.getElementById('ref_line').setAttribute('style', 'display: default;');
    }else{
        document.getElementById('ref').setAttribute('href', '#');
        document.getElementById('ref_line').setAttribute('style', 'display: none;');
    }
    art = new obj(400, 400);
    
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
        animation_number = requestAnimationFrame(animate);
    }
    
    animate();
}

function destroyAndRecreateCanvas(containerId, canvasId) {
    const oldCanvas = document.getElementById(canvasId);
    if (oldCanvas){
        if (animation_number){
            window.cancelAnimationFrame(animation_number);
            animation_number = null;
            try {
                const gl = oldCanvas.getContext('webgl2') || oldCanvas.getContext('webgl');
                if (gl) {
                    gl.getExtension('WEBGL_lose_context')?.loseContext();
                    
                    gl.useProgram(null);
                    gl.bindBuffer(gl.ARRAY_BUFFER, null);
                    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
                    gl.bindVertexArray(null);
                }
            } catch (e) {}
            if (art){
                art.destroy();
                art = null;
            }
        }
        oldCanvas.remove();
    }
    const newCanvas = document.createElement('canvas');
    newCanvas.id = canvasId;
    
    // newCanvas.width = oldCanvas.width;
    // newCanvas.height = oldCanvas.height;

    const container = document.getElementById(containerId);
    container.appendChild(newCanvas);

    return newCanvas;
}


async function setup() {
    const container = document.getElementById('arts');
    const keys = Object.keys(arts);
    keys.forEach(title => {
        const button = document.createElement('button');
        button.textContent = title;
        button.addEventListener('click', e => {
            run(title);
        });
        container.appendChild(button);
    });

    await init();
    const random_title = keys[Math.floor(Math.random() * keys.length)];
    run(random_title);
}

setup();
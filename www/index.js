import init, {
  JellyFish,
  Nudibranch,
  HeartController,
  PlanetaryTimer,
  ChristmasTree,
  Golfed1,
} from '../pkg/gagl_wasm.js';

var animation_number = null;
var art = null;
var arts = {
  Jellyfish: {
    class: JellyFish,
    ref: 'https://x.com/yuruyurau/status/1865420201086636376',
    text: null,
    color: [0.0, 1.0, 1.0],
  },
  Nudibranch: {
    class: Nudibranch,
    ref: 'https://x.com/yuruyurau/status/1866142306119885279',
    text: null,
    color: [0.0, 1.0, 1.0],
  },
  Heartbeat: {
    class: HeartController,
    ref: null,
    text: '<span style="color: rgb(255, 0.0, 74);">For my dearest <strong>Ghasedak</strong></span>',
    color: [1.0, 0.0, 0.29],
  },
  PlanetaryTimer: {
    class: PlanetaryTimer,
    ref: 'https://x.com/YoheiNishitsuji/status/1908486028018753622',
    text: null,
    color: [0.0, 0.0, 0.0],
  },
  ChristmasTree: {
    class: ChristmasTree,
    ref: 'https://x.com/YoheiNishitsuji/status/2004007970242547892',
    text: null,
    color: [1.0, 1.0, 1.0],
  },
  Golfed1: {
    class: Golfed1,
    ref: 'https://x.com/XorDev/status/2015813875833225715',
    text: null,
    color: [1.0, 1.0, 1.0],
  },
};

async function run(title) {
  destroyAndRecreateCanvas('canvas_container', 'canvas');
  let obj = arts[title]['class'];
  let color = arts[title]['color'];

  if (arts[title]['ref']) {
    document.getElementById('ref').setAttribute('href', arts[title]['ref']);
    document
      .getElementById('ref_line')
      .setAttribute('style', 'display: default;');
  } else {
    document.getElementById('ref').setAttribute('href', '#');
    document.getElementById('ref_line').setAttribute('style', 'display: none;');
  }
  if (arts[title]['text']) {
    document.getElementById('text_line').innerHTML = arts[title]['text'];
    document
      .getElementById('text_line')
      .setAttribute('style', 'display: default;');
  } else {
    document.getElementById('text_line').innerHTML = '';
    document
      .getElementById('text_line')
      .setAttribute('style', 'display: none;');
  }
  art = new obj(400, 400);

  const redSlider = document.getElementById('red');
  redSlider.value = color[0];
  const greenSlider = document.getElementById('green');
  greenSlider.value = color[1];
  const blueSlider = document.getElementById('blue');
  blueSlider.value = color[2];

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
  if (oldCanvas) {
    if (animation_number) {
      window.cancelAnimationFrame(animation_number);
      animation_number = null;
      try {
        const gl =
          oldCanvas.getContext('webgl2') || oldCanvas.getContext('webgl');
        if (gl) {
          gl.getExtension('WEBGL_lose_context')?.loseContext();

          gl.useProgram(null);
          gl.bindBuffer(gl.ARRAY_BUFFER, null);
          gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
          gl.bindVertexArray(null);
        }
      } catch (e) {}
      if (art) {
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
  keys.forEach((title) => {
    const button = document.createElement('a');
    button.textContent = title;
    button.setAttribute('href', '#' + title);
    container.appendChild(button);
  });

  function handleHash() {
    const title = window.location.hash.slice(1);
    if (title && Object.keys(arts).includes(title)) {
      run(title);
    }
  }

  window.addEventListener('hashchange', handleHash);

  await init();
  let title = window.location.hash.slice(1);
  if (!title || !keys.includes(title))
    title = keys[Math.floor(Math.random() * keys.length)];
  run(title);
}

setup();

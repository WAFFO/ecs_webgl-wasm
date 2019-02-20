(async () => {
    const webGL = await import('./wasm/client.js');

    const Engine = webGL.run();

    const renderLoop = () => {
        fps.render();
        Engine.tick();
        requestAnimationFrame(renderLoop);
    }

    var canvas = document.getElementById('canvas');

    document.addEventListener('mousedown', updateMouse, false);
    document.addEventListener('mouseup', updateMouse, false);
    document.addEventListener('mousemove', updateMouse, false);
    document.addEventListener('wheel', updateScroll, false);
    document.addEventListener('keydown', keyDown, false);
    document.addEventListener('keyup', keyUp, false);

    function updateMouse(e) {
        if (e.metaKey)
            Engine.mouse_click(e.buttons, e.movementX, e.movementY);
        else
            Engine.mouse_move(e.buttons, e.movementX, e.movementY);
    }
    function updateScroll(e) { Engine.mouse_scroll(e.deltaY); }
    function keyDown(e) { Engine.key_down(e.keyCode); }
    function keyUp(e) { Engine.key_up(e.keyCode); }

    requestAnimationFrame(renderLoop);
})();

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.innerHTML = `
Frames per Second:<br>
         latest = ${Math.round(fps)}<br>
avg of last 100 = ${Math.round(mean)}<br>
min of last 100 = ${Math.round(min)}<br>
max of last 100 = ${Math.round(max)}<br>
`;
  }
};
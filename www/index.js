// update this file name with name in Cargo.toml
(async () => {
    const webGL = await import('./wasm/ecs_webgl_wasm.js');

    const Engine = webGL.run();

    const renderLoop = () => {
        Engine.tick();
        requestAnimationFrame(renderLoop);
    }

    var canvas = document.getElementById('canvas');

    canvas.requestPointerLock = canvas.requestPointerLock;
    document.exitPointerLock = document.exitPointerLock;

    canvas.onclick = function() {
      canvas.requestPointerLock();
    };

    document.addEventListener('pointerlockchange', lockChangeAlert, false);
    document.addEventListener('mozpointerlockchange', lockChangeAlert, false);

    function lockChangeAlert() {
      if (document.pointerLockElement === canvas) {
        document.addEventListener("mousemove", updateMouse, false);
        document.addEventListener('keydown', keyDown, false);
        document.addEventListener('keyup', keyUp, false);
      } else {
        document.removeEventListener("mousemove", updateMouse, false);
        document.removeEventListener('keydown', keyDown, false);
        document.removeEventListener('keyup', keyUp, false);
      }
    }

    function updateMouse(e) { Engine.mouse_move(e.movementX, e.movementY); }
    function keyDown(e) { Engine.key_down(e.keyCode); }
    function keyUp(e) { Engine.key_up(e.keyCode); }

    requestAnimationFrame(renderLoop);
})();
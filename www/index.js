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
        document.addEventListener("mousemove", updatePosition, false);
      } else {
        document.removeEventListener("mousemove", updatePosition, false);
      }
    }

    function updatePosition(e) { Engine.mouse_move(e.movementX, e.movementY); }

    requestAnimationFrame(renderLoop);
})();
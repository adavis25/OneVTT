<script lang="ts">
  import { onMount } from 'svelte';
  import * as THREE from 'three';

  let container: HTMLDivElement;

  onMount(() => {
    // Scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x000000);

    // Camera — orthographic for top-down 2D view
    const aspect = container.clientWidth / container.clientHeight;
    let frustum = 10;
    const camera = new THREE.OrthographicCamera(
      -frustum * aspect,
       frustum * aspect,
       frustum,
      -frustum,
      0.1,
      1000
    );
    camera.position.set(0, 0, 10);
    camera.lookAt(0, 0, 0);

    // Renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(container.clientWidth, container.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // Basic grid — 200 units, 1 unit per cell, plenty of room to pan
    const grid = new THREE.GridHelper(40, 40, 0x444444, 0x333333);
    grid.rotation.x = Math.PI / 2;
    scene.add(grid);

    // Panning
    const canvas = renderer.domElement;
    canvas.style.cursor = 'grab';
    let isDragging = false;
    let lastPointerX = 0;
    let lastPointerY = 0;

    function onPointerDown(e: PointerEvent) {
      isDragging = true;
      lastPointerX = e.clientX;
      lastPointerY = e.clientY;
      canvas.setPointerCapture(e.pointerId);
      canvas.style.cursor = 'grabbing';
    }

    function onPointerMove(e: PointerEvent) {
      if (!isDragging) return;
      const dx = e.clientX - lastPointerX;
      const dy = e.clientY - lastPointerY;
      lastPointerX = e.clientX;
      lastPointerY = e.clientY;
      const scale = (frustum * 2) / container.clientHeight;
      camera.position.x -= dx * scale;
      camera.position.y += dy * scale;
    }

    function onPointerUp(e: PointerEvent) {
      isDragging = false;
      canvas.releasePointerCapture(e.pointerId);
      canvas.style.cursor = 'grab';
    }

    // Zooming — scroll wheel, zoom toward cursor
    const MIN_FRUSTUM = 2;
    const MAX_FRUSTUM = 80;
    const ZOOM_FACTOR = 1.1;

    function onWheel(e: WheelEvent) {
      e.preventDefault();

      const w = container.clientWidth;
      const h = container.clientHeight;

      // Cursor in NDC (-1..1), Y flipped
      const ndcX = (e.clientX / w) * 2 - 1;
      const ndcY = -(e.clientY / h * 2 - 1);

      // World position under cursor before zoom
      const curAspect = w / h;
      const worldX = camera.position.x + ndcX * frustum * curAspect;
      const worldY = camera.position.y + ndcY * frustum;

      frustum *= e.deltaY > 0 ? ZOOM_FACTOR : 1 / ZOOM_FACTOR;
      frustum = Math.max(MIN_FRUSTUM, Math.min(MAX_FRUSTUM, frustum));

      // Update camera frustum planes
      camera.left = -frustum * curAspect;
      camera.right = frustum * curAspect;
      camera.top = frustum;
      camera.bottom = -frustum;
      camera.updateProjectionMatrix();

      // Shift camera so the world point stays under the cursor
      camera.position.x = worldX - ndcX * frustum * curAspect;
      camera.position.y = worldY - ndcY * frustum;
    }

    canvas.addEventListener('pointerdown', onPointerDown);
    canvas.addEventListener('pointermove', onPointerMove);
    canvas.addEventListener('pointerup', onPointerUp);
    canvas.addEventListener('pointercancel', onPointerUp);
    canvas.addEventListener('wheel', onWheel, { passive: false });

    // Render loop
    let animationId: number;
    function animate() {
      animationId = requestAnimationFrame(animate);
      renderer.render(scene, camera);
    }
    animate();

    // Handle resize
    function onResize() {
      const w = container.clientWidth;
      const h = container.clientHeight;
      const aspect = w / h;
      camera.left = -frustum * aspect;
      camera.right = frustum * aspect;
      camera.top = frustum;
      camera.bottom = -frustum;
      camera.updateProjectionMatrix();
      renderer.setSize(w, h);
    }
    const resizeObserver = new ResizeObserver(onResize);
    resizeObserver.observe(container);

    // Cleanup
    return () => {
      cancelAnimationFrame(animationId);
      resizeObserver.disconnect();
      canvas.removeEventListener('pointerdown', onPointerDown);
      canvas.removeEventListener('pointermove', onPointerMove);
      canvas.removeEventListener('pointerup', onPointerUp);
      canvas.removeEventListener('pointercancel', onPointerUp);
      canvas.removeEventListener('wheel', onWheel);
      renderer.dispose();
      container.removeChild(renderer.domElement);
    };
  });
</script>

<div bind:this={container} class="canvas-container"></div>

<style>
  .canvas-container {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    user-select: none;
  }
</style>
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
    const frustum = 10;
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
    const grid = new THREE.GridHelper(200, 200, 0x444444, 0x333333);
    grid.rotation.x = Math.PI / 2;
    scene.add(grid);

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
  }
</style>
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import * as THREE from 'three';
	import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

	interface Props {
		sequence: string;
	}

	let { sequence }: Props = $props();

	let container: HTMLDivElement;
	let scene: THREE.Scene;
	let camera: THREE.PerspectiveCamera;
	let renderer: THREE.WebGLRenderer;
	let controls: OrbitControls;
	let helixGroup: THREE.Group;
	let frameId: number;

	const COLORS: Record<string, number> = {
		A: 0x4ADE80, // green-400
		C: 0x60A5FA, // blue-400
		G: 0xFACC15, // yellow-400
		T: 0xF87171  // red-400
	};

	onMount(() => {
		initScene();
		animate();
		
		window.addEventListener('resize', onWindowResize);
	});

	onDestroy(() => {
		if (frameId) cancelAnimationFrame(frameId);
		window.removeEventListener('resize', onWindowResize);
		if (renderer) {
			renderer.dispose();
			renderer.forceContextLoss();
		}
	});

	function initScene() {
		scene = new THREE.Scene();
		
		camera = new THREE.PerspectiveCamera(75, container.clientWidth / container.clientHeight, 0.1, 1000);
		camera.position.z = 15;
		camera.position.y = 5;

		renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
		renderer.setSize(container.clientWidth, container.clientHeight);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		container.appendChild(renderer.domElement);

		controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.autoRotate = true;
		controls.autoRotateSpeed = 2;

		const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
		scene.add(ambientLight);

		const pointLight = new THREE.PointLight(0xffffff, 1);
		pointLight.position.set(10, 10, 10);
		scene.add(pointLight);

		helixGroup = new THREE.Group();
		scene.add(helixGroup);

		createHelix();
	}

	function createHelix() {
		// Clear existing helix
		while(helixGroup.children.length > 0){ 
			helixGroup.remove(helixGroup.children[0]); 
		}

		if (!sequence) return;

		const numSteps = sequence.length;
		const radius = 3;
		const twist = 0.5;
		const stepHeight = 0.5;

		const sphereGeom = new THREE.SphereGeometry(0.3, 16, 16);
		const rungGeom = new THREE.CylinderGeometry(0.1, 0.1, radius * 2);

		for (let i = 0; i < numSteps; i++) {
			const angle = i * twist;
			const y = (i - numSteps / 2) * stepHeight;

			const base = sequence[i];
			const color = COLORS[base] || 0xffffff;
			const material = new THREE.MeshPhongMaterial({ 
				color, 
				emissive: color, 
				emissiveIntensity: 0.5,
				shininess: 100 
			});

			// Backbone A
			const xA = Math.cos(angle) * radius;
			const zA = Math.sin(angle) * radius;
			const sphereA = new THREE.Mesh(sphereGeom, material);
			sphereA.position.set(xA, y, zA);
			helixGroup.add(sphereA);

			// Backbone B
			const xB = Math.cos(angle + Math.PI) * radius;
			const zB = Math.sin(angle + Math.PI) * radius;
			const sphereB = new THREE.Mesh(sphereGeom, material);
			sphereB.position.set(xB, y, zB);
			helixGroup.add(sphereB);

			// Rung
			const rung = new THREE.Mesh(rungGeom, material);
			rung.position.set(0, y, 0);
			rung.rotation.z = Math.PI / 2;
			rung.rotation.y = angle;
			helixGroup.add(rung);
		}
	}

	$effect(() => {
		if (helixGroup && sequence) {
			createHelix();
		}
	});

	export function capture(): string {
		if (!renderer || !scene || !camera) return '';
		renderer.render(scene, camera);
		return renderer.domElement.toDataURL('image/png');
	}

	function onWindowResize() {
		if (!container || !camera || !renderer) return;
		camera.aspect = container.clientWidth / container.clientHeight;
		camera.updateProjectionMatrix();
		renderer.setSize(container.clientWidth, container.clientHeight);
	}

	function animate() {
		frameId = requestAnimationFrame(animate);
		if (controls) controls.update();
		if (renderer && scene && camera) renderer.render(scene, camera);
	}
</script>

<div bind:this={container} class="w-full h-full min-h-[400px]"></div>

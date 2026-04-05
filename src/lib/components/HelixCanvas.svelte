<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import * as THREE from 'three';
	import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

	interface Props {
		sequence: string;
		paused?: boolean;
	}

	let { sequence, paused = false }: Props = $props();

	let container: HTMLDivElement;
	let scene: THREE.Scene;
	let camera: THREE.PerspectiveCamera;
	let renderer: THREE.WebGLRenderer;
	let controls: OrbitControls;
	let helixGroup: THREE.Group;
	let particleGroup: THREE.Group;
	let frameId: number;

	const COLORS: Record<string, number> = {
		A: 0xFACC15, // Yellow
		T: 0xF87171, // Red
		G: 0x4ADE80, // Green
		C: 0x60A5FA  // Blue
	};

	const PAIRS: Record<string, string> = {
		A: 'T',
		T: 'A',
		G: 'C',
		C: 'G'
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
		
		camera = new THREE.PerspectiveCamera(40, container.clientWidth / container.clientHeight, 0.1, 1000);
		camera.position.z = 30;
		camera.position.y = 5;

		renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
		renderer.setSize(container.clientWidth, container.clientHeight);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		renderer.toneMapping = THREE.ReinhardToneMapping;
		renderer.toneMappingExposure = 1.2;
		container.appendChild(renderer.domElement);

		controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.autoRotate = true;
		controls.autoRotateSpeed = 0.8;
		controls.enablePan = false;

		// Lighting
		const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
		scene.add(ambientLight);

		const mainLight = new THREE.DirectionalLight(0xffffff, 1.5);
		mainLight.position.set(10, 20, 15);
		scene.add(mainLight);

		const rimLight = new THREE.PointLight(0x60A5FA, 2, 50);
		rimLight.position.set(-20, 10, -10);
		scene.add(rimLight);

		helixGroup = new THREE.Group();
		helixGroup.rotation.z = Math.PI / 2; // Initial horizontal orientation
		scene.add(helixGroup);

		particleGroup = new THREE.Group();
		scene.add(particleGroup);

		createEnvironment();
		createHelix();
	}

	function createEnvironment() {
		const particleCount = 200;
		const geometry = new THREE.SphereGeometry(0.05, 8, 8);
		const material = new THREE.MeshBasicMaterial({ 
			color: 0x60A5FA, 
			transparent: true, 
			opacity: 0.3 
		});

		for (let i = 0; i < particleCount; i++) {
			const particle = new THREE.Mesh(geometry, material);
			particle.position.set(
				(Math.random() - 0.5) * 60,
				(Math.random() - 0.5) * 60,
				(Math.random() - 0.5) * 60
			);
			particle.userData.velocity = new THREE.Vector3(
				(Math.random() - 0.5) * 0.02,
				(Math.random() - 0.5) * 0.02,
				(Math.random() - 0.5) * 0.02
			);
			particleGroup.add(particle);
		}
	}

	function createHelix() {
		while(helixGroup.children.length > 0){ 
			helixGroup.remove(helixGroup.children[0]); 
		}

		if (!sequence) return;

		const numSteps = sequence.length;
		const radius = 4.5;
		const twist = 0.4;
		const stepHeight = 0.9;
		const grooveOffset = 2.4; // Real DNA has Major and Minor grooves

		const backboneMaterial = new THREE.MeshStandardMaterial({ 
			color: 0x334155,
			roughness: 0.3,
			metalness: 0.2,
		});

		const pointsA: THREE.Vector3[] = [];
		const pointsB: THREE.Vector3[] = [];
		const beadGeom = new THREE.SphereGeometry(0.4, 12, 12);

		for (let i = 0; i < numSteps; i++) {
			const angle = i * twist;
			const y = (i - numSteps / 2) * stepHeight;

			// Backbone A
			const xA = Math.cos(angle) * radius;
			const zA = Math.sin(angle) * radius;
			const posA = new THREE.Vector3(xA, y, zA);
			pointsA.push(posA);

			// Backbone B (Offset for Major/Minor groove)
			const xB = Math.cos(angle + grooveOffset) * radius;
			const zB = Math.sin(angle + grooveOffset) * radius;
			const posB = new THREE.Vector3(xB, y, zB);
			pointsB.push(posB);

			// Phosphate Beads for texture
			if (i % 2 === 0) {
				const beadA = new THREE.Mesh(beadGeom, backboneMaterial);
				beadA.position.copy(posA);
				helixGroup.add(beadA);

				const beadB = new THREE.Mesh(beadGeom, backboneMaterial);
				beadB.position.copy(posB);
				helixGroup.add(beadB);
			}

			// Base Pairs
			const base1 = sequence[i];
			const base2 = PAIRS[base1] || 'A';

			const rungMaterial1 = new THREE.MeshStandardMaterial({ 
				color: COLORS[base1], 
				emissive: COLORS[base1],
				emissiveIntensity: 0.2,
				roughness: 0.4 
			});
			const rungMaterial2 = new THREE.MeshStandardMaterial({ 
				color: COLORS[base2], 
				emissive: COLORS[base2],
				emissiveIntensity: 0.2,
				roughness: 0.4 
			});

			// Calculate vector between backbones for rungs
			const rungVec = new THREE.Vector3().subVectors(posB, posA);
			const rungLen = rungVec.length();
			const rungGeom = new THREE.CylinderGeometry(0.25, 0.25, rungLen / 2);
			
			// Half-rung 1
			const hr1 = new THREE.Mesh(rungGeom, rungMaterial1);
			hr1.position.copy(posA).add(rungVec.clone().multiplyScalar(0.25));
			hr1.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), rungVec.clone().normalize());
			helixGroup.add(hr1);

			// Half-rung 2
			const hr2 = new THREE.Mesh(rungGeom, rungMaterial2);
			hr2.position.copy(posA).add(rungVec.clone().multiplyScalar(0.75));
			hr2.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), rungVec.clone().normalize());
			helixGroup.add(hr2);
		}

		// Smooth Backbone Tubes
		const curveA = new THREE.CatmullRomCurve3(pointsA);
		const tubeA = new THREE.Mesh(new THREE.TubeGeometry(curveA, numSteps * 4, 0.25, 8, false), backboneMaterial);
		helixGroup.add(tubeA);

		const curveB = new THREE.CatmullRomCurve3(pointsB);
		const tubeB = new THREE.Mesh(new THREE.TubeGeometry(curveB, numSteps * 4, 0.25, 8, false), backboneMaterial);
		helixGroup.add(tubeB);
	}

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
		
		if (helixGroup && !paused) {
			// Subtle multi-axis rotation for a more "floating" feel
			helixGroup.rotation.y += 0.005;
			helixGroup.rotation.z += 0.001;
		}

		if (particleGroup && !paused) {
			particleGroup.children.forEach(p => {
				p.position.add(p.userData.velocity);
				if (Math.abs(p.position.x) > 30) p.position.x *= -1;
				if (Math.abs(p.position.y) > 30) p.position.y *= -1;
				if (Math.abs(p.position.z) > 30) p.position.z *= -1;
			});
		}

		if (controls) {
			controls.autoRotate = !paused;
			controls.update();
		}
		
		if (renderer && scene && camera) renderer.render(scene, camera);
	}

	$effect(() => {
		if (helixGroup && sequence) {
			createHelix();
		}
	});
</script>

<div bind:this={container} class="w-full h-full min-h-[400px]"></div>

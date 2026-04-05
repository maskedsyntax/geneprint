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
		
		camera = new THREE.PerspectiveCamera(45, container.clientWidth / container.clientHeight, 0.1, 1000);
		camera.position.z = 25;
		camera.position.y = 10;

		renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
		renderer.setSize(container.clientWidth, container.clientHeight);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		renderer.shadowMap.enabled = true;
		container.appendChild(renderer.domElement);

		controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.autoRotate = true;
		controls.autoRotateSpeed = 1.5;

		const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
		scene.add(ambientLight);

		const dirLight = new THREE.DirectionalLight(0xffffff, 1.2);
		dirLight.position.set(10, 20, 10);
		dirLight.castShadow = true;
		scene.add(dirLight);

		const spotLight = new THREE.SpotLight(0xffffff, 0.8);
		spotLight.position.set(-10, -20, -10);
		scene.add(spotLight);

		helixGroup = new THREE.Group();
		scene.add(helixGroup);

		createHelix();
	}

	function createHelix() {
		while(helixGroup.children.length > 0){ 
			helixGroup.remove(helixGroup.children[0]); 
		}

		if (!sequence) return;

		const numSteps = sequence.length;
		const radius = 4;
		const twist = 0.5;
		const stepHeight = 0.8;

		const backboneMaterial = new THREE.MeshPhongMaterial({ 
			color: 0x2D3748, 
			shininess: 100,
			specular: 0x444444
		});

		const pointsA: THREE.Vector3[] = [];
		const pointsB: THREE.Vector3[] = [];

		for (let i = 0; i < numSteps; i++) {
			const angle = i * twist;
			const y = (i - numSteps / 2) * stepHeight;

			const xA = Math.cos(angle) * radius;
			const zA = Math.sin(angle) * radius;
			pointsA.push(new THREE.Vector3(xA, y, zA));

			const xB = Math.cos(angle + Math.PI) * radius;
			const zB = Math.sin(angle + Math.PI) * radius;
			pointsB.push(new THREE.Vector3(xB, y, zB));

			// Rungs - Split into two parts
			const base1 = sequence[i];
			const base2 = PAIRS[base1] || 'A';

			const rungMaterial1 = new THREE.MeshPhongMaterial({ color: COLORS[base1], shininess: 80 });
			const rungMaterial2 = new THREE.MeshPhongMaterial({ color: COLORS[base2], shininess: 80 });

			const rungGeom = new THREE.CylinderGeometry(0.2, 0.2, radius);
			
			// First half of rung
			const rung1 = new THREE.Mesh(rungGeom, rungMaterial1);
			rung1.position.set(Math.cos(angle) * (radius / 2), y, Math.sin(angle) * (radius / 2));
			rung1.rotation.z = Math.PI / 2;
			rung1.rotation.y = angle;
			helixGroup.add(rung1);

			// Second half of rung
			const rung2 = new THREE.Mesh(rungGeom, rungMaterial2);
			rung2.position.set(Math.cos(angle + Math.PI) * (radius / 2), y, Math.sin(angle + Math.PI) * (radius / 2));
			rung2.rotation.z = Math.PI / 2;
			rung2.rotation.y = angle;
			helixGroup.add(rung2);
		}

		// Create smooth backbone ribbons
		const curveA = new THREE.CatmullRomCurve3(pointsA);
		const tubeGeomA = new THREE.TubeGeometry(curveA, numSteps * 4, 0.35, 12, false);
		const meshA = new THREE.Mesh(tubeGeomA, backboneMaterial);
		helixGroup.add(meshA);

		const curveB = new THREE.CatmullRomCurve3(pointsB);
		const tubeGeomB = new THREE.TubeGeometry(curveB, numSteps * 4, 0.35, 12, false);
		const meshB = new THREE.Mesh(tubeGeomB, backboneMaterial);
		helixGroup.add(meshB);
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
		if (controls) controls.update();
		if (renderer && scene && camera) renderer.render(scene, camera);
	}

	$effect(() => {
		if (helixGroup && sequence) {
			createHelix();
		}
	});
</script>

<div bind:this={container} class="w-full h-full min-h-[400px]"></div>

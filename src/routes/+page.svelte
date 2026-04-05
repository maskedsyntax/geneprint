<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	// Use relative import or $lib alias for Vite to bundle correctly
	import init, { generate_dna_sequence, get_base_color } from '$lib/wasm/geneprint_wasm.js';
	import HelixCanvas from '$lib/components/HelixCanvas.svelte';
	import wasmUrl from '$lib/wasm/geneprint_wasm_bg.wasm?url';

	let identifier = $state('');
	let dnaSequence = $state<{ sequence: string; length: number } | null>(null);
	let wasmLoaded = $state(false);
	let canvasComponent: ReturnType<typeof HelixCanvas> | null = $state(null);

	onMount(async () => {
		if (browser) {
			try {
				// We need to pass the URL of the WASM file explicitly to init if it's managed by Vite
				await init(wasmUrl);
				wasmLoaded = true;
			} catch (e) {
				console.error('Failed to load WASM:', e);
			}
		}
	});

	$effect(() => {
		if (wasmLoaded && identifier.trim()) {
			const result = generate_dna_sequence(identifier, 32);
			dnaSequence = {
				sequence: result.sequence,
				length: result.length
			};
		} else {
			dnaSequence = null;
		}
	});

	function exportImage() {
		if (canvasComponent) {
			const dataUrl = canvasComponent.capture();
			if (dataUrl) {
				const link = document.createElement('a');
				link.download = `geneprint-${identifier || 'sequence'}.png`;
				link.href = dataUrl;
				link.click();
			}
		}
	}
</script>

<div class="min-h-screen bg-slate-950 text-slate-100 flex flex-col overflow-hidden">
	{#if !wasmLoaded}
		<div class="fixed inset-0 z-[100] bg-slate-950 flex flex-col items-center justify-center space-y-6">
			<div class="w-16 h-16 border-4 border-slate-800 border-t-blue-500 rounded-full animate-spin"></div>
			<div class="text-center space-y-2">
				<h1 class="text-2xl font-black tracking-tighter animate-pulse text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">
					Initializing GenePrint
				</h1>
				<p class="text-xs font-mono text-slate-600 uppercase tracking-widest">Loading Rust WASM Modules</p>
			</div>
		</div>
	{/if}

	<!-- Background Effects -->
	<div class="fixed inset-0 pointer-events-none opacity-20">
		<div class="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-blue-500 rounded-full blur-[120px]"></div>
		<div class="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-green-500 rounded-full blur-[120px]"></div>
	</div>

	<header class="relative z-10 p-6 flex justify-between items-center border-b border-slate-800/50 backdrop-blur-md bg-slate-950/50">
		<div class="flex items-center space-x-3">
			<div class="w-10 h-10 bg-gradient-to-br from-green-400 to-blue-600 rounded-xl flex items-center justify-center shadow-lg shadow-blue-900/20">
				<span class="font-black text-xl">G</span>
			</div>
			<h1 class="text-2xl font-black tracking-tighter">
				GenePrint
			</h1>
		</div>
		<div class="text-xs font-mono text-slate-500 tracking-widest uppercase bg-slate-900 px-3 py-1 rounded-full border border-slate-800">
			v1.0.0-experimental
		</div>
	</header>

	<main class="relative z-10 flex-1 flex flex-col md:flex-row overflow-hidden">
		<!-- Left Panel: Controls & Sequence -->
		<div class="w-full md:w-[450px] p-8 space-y-8 flex flex-col border-r border-slate-800/50 bg-slate-900/20 backdrop-blur-xl">
			<section class="space-y-4">
				<div class="space-y-1">
					<h2 class="text-sm font-bold uppercase tracking-widest text-slate-500">Seed Identifier</h2>
					<p class="text-xs text-slate-600 italic">Unique string to derive your DNA signature.</p>
				</div>
				<div class="relative group">
					<input
						type="text"
						bind:value={identifier}
						placeholder="Username, email, or string..."
						class="w-full bg-slate-900/80 border border-slate-800 rounded-xl px-4 py-4 text-lg focus:outline-none focus:border-blue-500/50 focus:ring-4 focus:ring-blue-500/10 transition-all placeholder:text-slate-700 shadow-inner"
					/>
				</div>
			</section>

			{#if dnaSequence}
				<section class="flex-1 space-y-6 overflow-hidden flex flex-col">
					<div class="space-y-4">
						<div class="flex justify-between items-end">
							<h2 class="text-sm font-bold uppercase tracking-widest text-slate-500">DNA Signature</h2>
							<span class="text-[10px] font-mono text-blue-400 bg-blue-400/10 px-2 py-0.5 rounded border border-blue-400/20">DETERMINISTIC</span>
						</div>
						<div class="bg-slate-900/80 border border-slate-800 rounded-xl p-6 font-mono text-lg break-all leading-relaxed tracking-widest shadow-inner overflow-y-auto max-h-[300px] scrollbar-hide">
							{#each dnaSequence.sequence as base}
								<span style="color: {get_base_color(base)}" class="transition-colors duration-300">{base}</span>
							{/each}
						</div>
					</div>

					<div class="grid grid-cols-2 gap-4">
						<div class="bg-slate-900/80 border border-slate-800 rounded-xl p-4 space-y-1 shadow-sm">
							<span class="block text-[10px] font-bold text-slate-600 uppercase">Length</span>
							<span class="text-xl font-black text-slate-200">{dnaSequence.length} <span class="text-xs font-medium text-slate-500">bp</span></span>
						</div>
						<div class="bg-slate-900/80 border border-slate-800 rounded-xl p-4 space-y-1 shadow-sm">
							<span class="block text-[10px] font-bold text-slate-600 uppercase">Complexity</span>
							<span class="text-xl font-black text-slate-200">High</span>
						</div>
					</div>

					<div class="pt-4 grid grid-cols-2 gap-4">
						<button 
							class="bg-slate-800 text-slate-200 font-bold py-3 rounded-xl hover:bg-slate-700 active:scale-[0.98] transition-all border border-slate-700"
							onclick={() => navigator.clipboard.writeText(dnaSequence?.sequence || '')}
						>
							Copy Text
						</button>
						<button 
							class="bg-slate-100 text-slate-950 font-bold py-3 rounded-xl hover:bg-white active:scale-[0.98] transition-all shadow-xl shadow-white/5"
							onclick={exportImage}
						>
							Export PNG
						</button>
					</div>
				</section>
			{:else}
				<div class="flex-1 flex items-center justify-center text-center p-12 border-2 border-dashed border-slate-800/50 rounded-3xl">
					<div class="space-y-3">
						<div class="w-12 h-12 border-4 border-slate-800 border-t-blue-500 rounded-full animate-spin mx-auto opacity-20"></div>
						<p class="text-sm font-medium text-slate-600 uppercase tracking-widest">Awaiting Input</p>
					</div>
				</div>
			{/if}
		</div>

		<!-- Right Panel: 3D Visualization -->
		<div class="flex-1 relative bg-slate-950">
			{#if dnaSequence}
				<div class="absolute inset-0">
					<HelixCanvas bind:this={canvasComponent} sequence={dnaSequence.sequence} />
				</div>
				<div class="absolute bottom-8 left-1/2 -translate-x-1/2 bg-slate-900/80 backdrop-blur-md border border-slate-800 px-6 py-3 rounded-full flex items-center space-x-6 text-[10px] font-bold uppercase tracking-widest text-slate-400 shadow-2xl">
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 rounded-full bg-[#FACC15]"></div>
						<span>Adenine</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 rounded-full bg-[#F87171]"></div>
						<span>Thymine</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 rounded-full bg-[#4ADE80]"></div>
						<span>Guanine</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 rounded-full bg-[#60A5FA]"></div>
						<span>Cytosine</span>
					</div>
				</div>
			{:else}
				<div class="absolute inset-0 flex items-center justify-center">
					<div class="text-center space-y-4">
						<div class="text-8xl font-black text-slate-900 select-none">GENE</div>
						<p class="text-slate-700 text-sm font-medium uppercase tracking-[0.5em]">Interactive 3D Preview</p>
					</div>
				</div>
			{/if}
		</div>
	</main>

	<footer class="relative z-10 p-4 text-center border-t border-slate-800/50 bg-slate-950/50 backdrop-blur-md">
		<p class="text-[10px] text-slate-600 uppercase tracking-[0.2em]">
			Generated with Rust WASM + Three.js &bull; Open Source DNA signature tool
		</p>
	</footer>
</div>

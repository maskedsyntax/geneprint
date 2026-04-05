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
	let isPaused = $state(false);

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

<div class="min-h-screen bg-[#0a0b0e] text-zinc-300 flex flex-col overflow-hidden font-sans selection:bg-zinc-500/20">
	{#if !wasmLoaded}
		<div class="fixed inset-0 z-[100] bg-[#0a0b0e] flex flex-col items-center justify-center space-y-6">
			<div class="w-10 h-10 border-2 border-zinc-900 border-t-zinc-400 rounded-full animate-spin"></div>
			<div class="text-center space-y-2">
				<h1 class="text-lg font-bold tracking-tight text-zinc-100">
					Initializing Geneprint
				</h1>
				<p class="text-[10px] font-mono text-zinc-700 uppercase tracking-widest">Protocol v1.2</p>
			</div>
		</div>
	{/if}

	<!-- Subdued Neutral Gradients -->
	<div class="fixed inset-0 pointer-events-none opacity-20">
		<div class="absolute top-[20%] left-[10%] w-[30%] h-[30%] bg-zinc-800/10 rounded-full blur-[100px]"></div>
		<div class="absolute bottom-[20%] right-[10%] w-[30%] h-[30%] bg-zinc-700/10 rounded-full blur-[100px]"></div>
	</div>

	<header class="relative z-10 px-10 py-6 flex justify-between items-center border-b border-zinc-800/40 bg-[#0a0b0e]/80 backdrop-blur-md">
		<div class="flex items-center space-x-5">
			<div class="w-10 h-10 flex items-center justify-center">
				<img src="/logo.png" alt="GenePrint Logo" class="w-full h-full object-contain" />
			</div>
			<h1 class="text-sm font-bold tracking-[0.2em] text-zinc-100 uppercase">
				Geneprint
			</h1>
		</div>
		<div class="flex items-center space-x-6 text-[10px] font-bold uppercase tracking-[0.1em] text-zinc-600">
			<span>Station: 01</span>
			<div class="flex items-center space-x-1.5">
				<div class="w-1.5 h-1.5 rounded-full bg-zinc-700 animate-pulse"></div>
				<span>Processing</span>
			</div>
		</div>
	</header>

	<main class="relative z-10 flex-1 flex flex-col md:flex-row overflow-hidden">
		<!-- Workspace Controls -->
		<div class="w-full md:w-[450px] p-10 space-y-12 flex flex-col border-r border-zinc-800/40 bg-[#0a0b0e]/30 backdrop-blur-3xl overflow-y-auto scrollbar-hide">
			<section class="space-y-4">
				<h2 class="text-[10px] font-bold uppercase tracking-[0.2em] text-zinc-500">Identifier</h2>
				<input
					type="text"
					bind:value={identifier}
					placeholder="Enter specimen ID..."
					class="obsidian-input w-full rounded-xl px-5 py-4 text-sm font-medium"
				/>
				<p class="text-[10px] text-zinc-700 leading-relaxed uppercase tracking-widest">
					Deterministic mapping protocol active. Sequence derived from input sample.
				</p>
			</section>

			{#if dnaSequence}
				<section class="flex-1 space-y-10 flex flex-col animate-in fade-in duration-500">
					<div class="space-y-4">
						<h2 class="text-[10px] font-bold uppercase tracking-[0.2em] text-zinc-500">Signature</h2>
						<div class="obsidian-panel p-8 font-mono text-xl break-all leading-relaxed tracking-[0.2em] max-h-[250px] overflow-y-auto scrollbar-hide border-zinc-800/30">
							{#each dnaSequence.sequence as base}
								<span style="color: {get_base_color(base)}" class="hover:brightness-110 cursor-default transition-all opacity-90">{base}</span>
							{/each}
						</div>
					</div>

					<div class="grid grid-cols-2 gap-8">
						<div class="space-y-1">
							<span class="block text-[10px] font-bold text-zinc-700 uppercase tracking-widest">Helix Pairs</span>
							<span class="text-xl font-bold text-zinc-300">{dnaSequence.length}</span>
						</div>
						<div class="space-y-1">
							<span class="block text-[10px] font-bold text-zinc-700 uppercase tracking-widest">Method</span>
							<span class="text-xl font-bold text-zinc-300">Deterministic</span>
						</div>
					</div>

					<div class="pt-6 grid grid-cols-2 gap-4">
						<button class="btn-secondary" onclick={() => navigator.clipboard.writeText(dnaSequence?.sequence || '')}>
							Copy
						</button>
						<button class="btn-primary" onclick={exportImage}>
							Export
						</button>
					</div>
				</section>
			{:else}
				<div class="flex-1 flex items-center justify-center p-12 border border-zinc-800/50 border-dashed rounded-2xl opacity-40">
					<div class="text-center space-y-4">
						<div class="w-10 h-10 border-2 border-zinc-900 border-t-zinc-700 rounded-full animate-spin mx-auto"></div>
						<p class="text-[10px] font-bold text-zinc-700 uppercase tracking-[0.2em]">Ready For Input</p>
					</div>
				</div>
			{/if}

			<div class="pt-4 mt-auto">
				<div class="p-5 bg-zinc-950/40 border border-zinc-900/50 rounded-2xl flex items-start space-x-4">
					<div class="w-1.5 h-1.5 rounded-full bg-zinc-600 mt-1.5"></div>
					<p class="text-[10px] text-zinc-600 leading-relaxed uppercase tracking-widest">
						Bio-signature mapping complete. Each sample correlates to a unique reproducible sequence.
					</p>
				</div>
			</div>
		</div>

		<!-- Observation Chamber -->
		<div class="flex-1 relative bg-zinc-950/20 overflow-hidden">
			{#if dnaSequence}
				<div class="absolute inset-0">
					<HelixCanvas bind:this={canvasComponent} sequence={dnaSequence.sequence} paused={isPaused} />
				</div>
				
				<!-- Chambers HUD Overlay -->
				<div class="absolute top-10 right-10 z-20">
					<button 
						class="bg-zinc-900/60 backdrop-blur-xl border border-zinc-800 p-3.5 rounded-full hover:bg-zinc-800 transition-all text-zinc-400 group shadow-2xl"
						onclick={() => isPaused = !isPaused}
						title={isPaused ? "Play" : "Pause"}
					>
						{#if isPaused}
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 group-hover:scale-110 transition-transform" viewBox="0 0 24 24" fill="currentColor">
								<path d="M8 5v14l11-7z"/>
							</svg>
						{:else}
							<svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 group-hover:scale-110 transition-transform" viewBox="0 0 24 24" fill="currentColor">
								<path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
							</svg>
						{/if}
					</button>
				</div>

				<!-- Base HUD -->
				<div class="absolute bottom-12 left-1/2 -translate-x-1/2 flex items-center space-x-10 px-10 py-4 obsidian-panel border-zinc-800/40 z-20">
					{#each ['A', 'T', 'G', 'C'] as base}
						<div class="flex items-center space-x-3 group">
							<div class="w-2 h-2 rounded-full shadow-[0_0_10px_rgba(255,255,255,0.05)] transition-transform group-hover:scale-125" style="background-color: {get_base_color(base)}"></div>
							<span class="text-[11px] font-bold text-zinc-500 tracking-[0.2em]">{base}</span>
						</div>
					{/each}
				</div>
			{:else}
				<div class="absolute inset-0 flex items-center justify-center opacity-[0.03] select-none">
					<div class="text-center space-y-6">
						<div class="text-[12rem] font-black text-zinc-100 tracking-[0.2em]">PROTO</div>
						<p class="text-zinc-100 text-[10px] font-bold uppercase tracking-[1.5em]">System Scanning</p>
					</div>
				</div>
			{/if}
		</div>
	</main>

	<footer class="relative z-10 px-10 py-5 border-t border-zinc-800/40 bg-[#0a0b0e]/80 backdrop-blur-md">
		<p class="text-[9px] text-zinc-600 uppercase tracking-[0.5em] text-center font-bold">
			Molecular Derivation &bull; v1.2.0 &bull; Private Environment
		</p>
	</footer>
</div>

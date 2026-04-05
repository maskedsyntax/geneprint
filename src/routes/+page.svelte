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

<div class="min-h-screen bg-[#040d12] text-emerald-50 flex flex-col overflow-hidden font-sans selection:bg-emerald-500/30">
	{#if !wasmLoaded}
		<div class="fixed inset-0 z-[100] bg-[#040d12] flex flex-col items-center justify-center space-y-6">
			<div class="w-16 h-16 border-4 border-emerald-950 border-t-emerald-500 rounded-full animate-spin"></div>
			<div class="text-center space-y-2">
				<h1 class="text-2xl font-black tracking-tighter animate-pulse text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-blue-500">
					Initializing Bio-Lab
				</h1>
				<p class="text-[10px] font-mono text-emerald-800 uppercase tracking-[0.3em]">Sequencing Core Engine</p>
			</div>
		</div>
	{/if}

	<!-- Microscopic Dust Background Effects -->
	<div class="fixed inset-0 pointer-events-none overflow-hidden">
		<div class="absolute top-[10%] left-[15%] w-[30%] h-[30%] bg-emerald-900/10 rounded-full blur-[120px] animate-pulse"></div>
		<div class="absolute bottom-[15%] right-[20%] w-[30%] h-[30%] bg-blue-900/10 rounded-full blur-[120px] animate-pulse"></div>
	</div>

	<header class="relative z-10 p-6 flex justify-between items-center border-b border-emerald-500/10 backdrop-blur-md bg-emerald-950/10">
		<div class="flex items-center space-x-3">
			<div class="w-10 h-10 bg-gradient-to-br from-emerald-400 to-emerald-700 rounded-xl flex items-center justify-center shadow-lg shadow-emerald-900/20">
				<span class="font-black text-xl text-emerald-950">G</span>
			</div>
			<div>
				<h1 class="text-xl font-black tracking-tighter text-emerald-50 uppercase leading-tight">
					GenePrint
				</h1>
				<p class="text-[8px] font-bold text-emerald-600 uppercase tracking-[0.2em] -mt-0.5">Molecular Sequencer</p>
			</div>
		</div>
		<div class="flex items-center space-x-4">
			<div class="text-[10px] font-mono text-emerald-500/60 tracking-widest uppercase bg-emerald-950/40 px-3 py-1 rounded-full border border-emerald-500/10">
				Diagnostic Node: ALPHA-7
			</div>
		</div>
	</header>

	<main class="relative z-10 flex-1 flex flex-col md:flex-row overflow-hidden">
		<!-- Left Panel: Lab Controls -->
		<div class="w-full md:w-[480px] p-8 space-y-10 flex flex-col border-r border-emerald-500/10 bg-emerald-950/5 backdrop-blur-3xl overflow-y-auto scrollbar-hide">
			<section class="space-y-4">
				<div class="flex items-center space-x-2">
					<div class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></div>
					<h2 class="text-[10px] font-bold uppercase tracking-[0.4em] text-emerald-700">Sequence Input</h2>
				</div>
				<div class="relative group">
					<input
						type="text"
						bind:value={identifier}
						placeholder="Unique specimen ID..."
						class="w-full bg-emerald-950/30 border border-emerald-500/20 rounded-2xl px-6 py-5 text-lg focus:outline-none focus:border-emerald-400/50 focus:ring-4 focus:ring-emerald-400/5 transition-all placeholder:text-emerald-900 text-emerald-100 shadow-inner font-mono"
					/>
				</div>
				<p class="text-[10px] text-emerald-800 leading-relaxed uppercase tracking-wider pl-1">
					Enter any unique string to derive a deterministic DNA signature for the specimen.
				</p>
			</section>

			{#if dnaSequence}
				<section class="flex-1 space-y-8 animate-in fade-in slide-in-from-left-4 duration-700 flex flex-col">
					<div class="space-y-4">
						<div class="flex justify-between items-center px-1">
							<h2 class="text-[10px] font-bold uppercase tracking-[0.4em] text-emerald-700">Derived Nucleotides</h2>
							<div class="flex items-center space-x-2">
								<span class="text-[8px] font-bold text-emerald-500 px-2 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 tracking-tighter">AUTHENTICATED</span>
							</div>
						</div>
						<div class="bio-panel p-8 font-mono text-xl break-all leading-relaxed tracking-[0.3em] shadow-inner max-h-[280px] overflow-y-auto scrollbar-hide border-emerald-500/10">
							{#each dnaSequence.sequence as base}
								<span style="color: {get_base_color(base)}" class="transition-all duration-500 hover:brightness-125 cursor-default">{base}</span>
							{/each}
						</div>
					</div>

					<div class="grid grid-cols-2 gap-4">
						<div class="bg-emerald-950/30 border border-emerald-500/10 rounded-2xl p-5 space-y-1 shadow-sm group hover:border-emerald-500/30 transition-all">
							<span class="block text-[8px] font-bold text-emerald-700 uppercase tracking-widest">Base Pairs</span>
							<span class="text-2xl font-black text-emerald-100">{dnaSequence.length} <span class="text-[10px] font-medium text-emerald-700">BP</span></span>
						</div>
						<div class="bg-emerald-950/30 border border-emerald-500/10 rounded-2xl p-5 space-y-1 shadow-sm group hover:border-emerald-500/30 transition-all">
							<span class="block text-[8px] font-bold text-emerald-700 uppercase tracking-widest">Integrity</span>
							<span class="text-2xl font-black text-emerald-100">100<span class="text-sm font-medium text-emerald-700">%</span></span>
						</div>
					</div>

					<div class="pt-6 grid grid-cols-2 gap-4">
						<button 
							class="bg-emerald-900/20 text-emerald-400 font-black py-4 rounded-2xl hover:bg-emerald-900/40 active:scale-[0.98] transition-all border border-emerald-500/10 uppercase text-[10px] tracking-widest"
							onclick={() => navigator.clipboard.writeText(dnaSequence?.sequence || '')}
						>
							Copy Text
						</button>
						<button 
							class="bg-emerald-500 text-emerald-950 font-black py-4 rounded-2xl hover:bg-emerald-400 active:scale-[0.98] transition-all shadow-xl shadow-emerald-500/10 uppercase text-[10px] tracking-widest"
							onclick={exportImage}
						>
							Export PNG
						</button>
					</div>
				</section>
			{:else}
				<div class="flex-1 flex items-center justify-center text-center p-12 border-2 border-dashed border-emerald-900/30 rounded-[3rem] animate-pulse">
					<div class="space-y-4">
						<div class="w-16 h-16 border-4 border-emerald-950 border-t-emerald-700 rounded-full animate-spin mx-auto opacity-40"></div>
						<div>
							<p class="text-[10px] font-bold text-emerald-900 uppercase tracking-[0.5em]">System Idle</p>
							<p class="text-[8px] font-medium text-emerald-950 uppercase tracking-[0.2em] mt-1">Ready for Sampling</p>
						</div>
					</div>
				</div>
			{/if}

			<div class="pt-4 mt-auto">
				<div class="bg-blue-500/5 border border-blue-500/10 p-4 rounded-2xl flex items-start space-x-3">
					<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-blue-500 mt-0.5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<p class="text-[9px] text-blue-300/60 leading-relaxed uppercase tracking-wide">
						Deterministic mapping established. Every identifier correlates to a unique and reproducible genetic signature.
					</p>
				</div>
			</div>
		</div>

		<!-- Right Panel: Visualization Chamber -->
		<div class="flex-1 relative bg-[#040d12]">
			<div class="absolute inset-0 bg-gradient-to-t from-emerald-950/20 to-transparent pointer-events-none"></div>
			
			{#if dnaSequence}
				<div class="absolute inset-0">
					<HelixCanvas bind:this={canvasComponent} sequence={dnaSequence.sequence} paused={isPaused} />
				</div>
				
				<!-- Lab HUD Controls -->
				<div class="absolute top-8 right-8 flex flex-col space-y-3 z-20">
					<button 
						class="bg-emerald-950/80 backdrop-blur-xl border border-emerald-500/20 p-4 rounded-2xl hover:bg-emerald-900/90 transition-all text-emerald-400 shadow-2xl group"
						onclick={() => isPaused = !isPaused}
						title={isPaused ? "Resume Sequence" : "Pause Sequence"}
					>
						{#if isPaused}
							<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 group-hover:scale-110 transition-transform" viewBox="0 0 24 24" fill="currentColor">
								<path d="M8 5v14l11-7z"/>
							</svg>
						{:else}
							<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 group-hover:scale-110 transition-transform" viewBox="0 0 24 24" fill="currentColor">
								<path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
							</svg>
						{/if}
					</button>
				</div>

				<!-- Laboratory HUD Data -->
				<div class="absolute bottom-12 left-1/2 -translate-x-1/2 flex flex-col items-center space-y-4 z-20 w-full max-w-lg">
					<div class="flex items-center space-x-8 px-10 py-4 bg-emerald-950/60 backdrop-blur-2xl border border-emerald-500/10 rounded-full shadow-2xl">
						<div class="flex items-center space-x-2 group">
							<div class="w-2.5 h-2.5 rounded-full bg-[#FACC15] shadow-[0_0_10px_rgba(250,204,21,0.3)] group-hover:scale-125 transition-transform"></div>
							<span class="text-[9px] font-black uppercase tracking-widest text-emerald-200">A</span>
						</div>
						<div class="flex items-center space-x-2 group">
							<div class="w-2.5 h-2.5 rounded-full bg-[#F87171] shadow-[0_0_10px_rgba(248,113,113,0.3)] group-hover:scale-125 transition-transform"></div>
							<span class="text-[9px] font-black uppercase tracking-widest text-emerald-200">T</span>
						</div>
						<div class="flex items-center space-x-2 group">
							<div class="w-2.5 h-2.5 rounded-full bg-[#4ADE80] shadow-[0_0_10px_rgba(74,222,128,0.3)] group-hover:scale-125 transition-transform"></div>
							<span class="text-[9px] font-black uppercase tracking-widest text-emerald-200">G</span>
						</div>
						<div class="flex items-center space-x-2 group">
							<div class="w-2.5 h-2.5 rounded-full bg-[#60A5FA] shadow-[0_0_10px_rgba(96,165,250,0.3)] group-hover:scale-125 transition-transform"></div>
							<span class="text-[9px] font-black uppercase tracking-widest text-emerald-200">C</span>
						</div>
					</div>
				</div>
			{:else}
				<div class="absolute inset-0 flex items-center justify-center">
					<div class="text-center space-y-6">
						<div class="text-9xl font-black text-emerald-950/20 select-none tracking-[0.2em]">CELL</div>
						<p class="text-emerald-900/40 text-[10px] font-bold uppercase tracking-[1em]">Scanning Chamber</p>
					</div>
				</div>
			{/if}
		</div>
	</main>

	<footer class="relative z-10 p-4 text-center border-t border-emerald-500/10 bg-emerald-950/20 backdrop-blur-md">
		<p class="text-[9px] text-emerald-800 uppercase tracking-[0.4em] font-medium">
			Quantum-Biological Engine &bull; Deterministic Derivation v1.2 &bull; No Storage Persistence
		</p>
	</footer>
</div>

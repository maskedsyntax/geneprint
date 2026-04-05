Project Plan: GenePrint – DNA Signature Generator
This document outlines a comprehensive, structured plan for developing an entertaining, deterministic DNA signature application. The core objective is to transform any unique identifier (such as an email address, username, phone number, or custom string) into a reproducible fake DNA sequence accompanied by a stunning interactive 3D helix visualization. The output is mathematically derived solely from the input, ensuring identical identifiers always produce identical results. No data is stored, transmitted, or persisted beyond the current browser session.
The project emphasizes a modern, experimental tech stack while remaining lightweight and client-side only.
1. Project Scope and Objectives
	•	Primary Goal: Deliver a deterministic generator that converts any string identifier into a fixed-length DNA sequence (A, C, G, T bases) and a high-fidelity interactive 3D DNA helix visualization.
	•	Key Constraints: Entirely client-side (no backend server), 100% deterministic behavior, entertainment-focused, with graceful fallback for older browsers.
	•	Target Platform: Modern web application (progressive web app optional).
	•	Tech Stack:
	◦	UI Framework: SvelteKit (Svelte 5 with Runes)
	◦	Core Generation Logic: Rust compiled to WebAssembly (wasm-pack + wasm-bindgen)
	◦	3D Visualization: Three.js using WebGPURenderer (with automatic WebGL fallback)
	◦	Styling: Tailwind CSS
	◦	Build Tool: Vite (integrated in SvelteKit)
	•	Success Criteria: The application must produce consistent, high-quality outputs; offer smooth interactive 3D visualization; remain fully deterministic; and provide intuitive export options. The experience should feel noticeably premium and experimental.
2. High-Level Development Plan
The project is divided into five sequential phases. Effort estimates assume solo development with intermediate knowledge of JavaScript/TypeScript and basic Rust familiarity. The experimental elements (Rust WASM and WebGPU) add some initial setup time but deliver significant visual and performance impact.
Phase 1: Requirements Gathering & Architecture (2–3 days)
	•	Define input types, validation rules, and deterministic requirements.
	•	Design the architecture:
	◦	Input string → Rust WASM module (hashing + seeded PRNG + DNA sequence generation).
	◦	Sequence → SvelteKit component → Three.js WebGPURenderer for 3D helix rendering (colored base-pair rungs driven by the sequence).
	•	Research and prototype basic Rust-to-WASM integration and Three.js WebGPU setup.
	•	Decide on graceful degradation (WebGL fallback if WebGPU is unavailable).
	•	Deliverable: Updated architecture diagram (Identifier → Rust WASM → Sequence → Three.js Scene) and technical specification for the generation algorithm.
Phase 2: Design (2–4 days)
	•	Create UI wireframes and component structure in SvelteKit (input form, result panel, interactive 3D canvas).
	•	Design the 3D helix aesthetics: smooth double helix with glowing, sequence-driven base-pair rungs (A=green, C=blue, G=yellow, T=red), camera controls (orbit, zoom, pan), subtle lighting, and optional gentle animation.
	•	Plan sequence formatting and UI interactions (copy sequence, export options).
	•	Define color schemes, typography, and responsive layout.
	•	Deliverable: Svelte component sketches, 3D scene concept, and detailed technical notes on Rust WASM API and Three.js node materials/shaders.
Phase 3: Implementation (7–12 days)
	•	Set up the SvelteKit project with Tailwind CSS.
	•	Develop the Rust WASM module: implement high-quality hashing and Mulberry32-style seeded PRNG, expose clean functions to JavaScript (e.g., generateDNASequence(identifier: string, length: number)).
	•	Integrate Rust WASM into SvelteKit (using wasm-pack and appropriate Vite plugins).
	•	Implement core UI in Svelte: identifier input, result display, and controls.
	•	Build the interactive 3D helix using Three.js WebGPURenderer: create helix geometry, dynamic materials for rungs based on the DNA sequence, lighting, and user controls (OrbitControls).
	•	Add export functionality (PNG of current view, text sequence, optional animated GIF).
	•	Implement fallback logic for WebGL when WebGPU is unsupported.
	•	Deliverable: Fully functional application with interactive 3D visualization.
Phase 4: Testing & Refinement (3–5 days)
	•	Verify determinism: identical inputs must produce identical sequences and 3D helix (test across devices and browsers).
	•	Test WebGPU performance and fallback behavior on multiple browsers (Chrome, Edge, Firefox, Safari).
	•	Validate edge cases (empty input, very long strings, special characters).
	•	Performance and usability testing of the 3D scene (rotation, zoom, mobile touch support).
	•	Gather informal feedback and refine visuals/interactions.
	•	Deliverable: Stable, bug-free version with polished user experience.
Phase 5: Polish & Deployment (2–3 days)
	•	Add final enhancements: subtle animations, loading states for WASM initialization, help/tooltips explaining determinism, dark/light theme support.
	•	Optimize bundle size and WASM loading.
	•	Make the app installable as a Progressive Web App (PWA).
	•	Deploy to GitHub Pages, Vercel, or Netlify (ensure WASM assets are correctly served).
	•	Prepare repository with clear README, including setup instructions for Rust and WASM.
	•	Deliverable: Production-ready application, live deployment, and complete GitHub repository.


<template>
	<div class="flex flex-col min-h-screen justify-center items-center">
		<h1>Version: {{ version_application }}</h1>
		<h1>Release: 03/12/2024</h1>
		<h1>Tauri Version: {{ version_tauri }}</h1>
		<h1>Rust Version: {{ version_rust }}</h1>
		<h1>Author: {{ author }}</h1>
		<h1>OS: {{ target }}</h1>
		<p class="text-xs p-2">Description: {{ description }}</p>
	</div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core'
import Package from '../types/package';

export default {
	/**
		 * Grab the Rust Version Information
		 */
	async beforeCreate() {
		const msg = await invoke('package_version') as Package;

		this.version_application = msg.version;
		
		if (typeof msg.dependencies['tauri'] === 'object') {
			this.version_tauri = msg.dependencies['tauri'].version;
		} else {
			this.version_tauri = '';
		}

		this.version_rust = msg.rust_version;
		this.author = msg.authors[0];
		this.description = msg.description;
		this.target = msg.target;
	},
	data() {
		return {
			version_application: '',
			version_tauri: '',
			author: '',
			description: '',
			version_rust: '',
			target: ''
		}
	}
}
</script>
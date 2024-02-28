<template>
	<div class="relative h-full cursor-default">
		<button class="rounded text-sm cursor-default flex items-center leading-tight justify-center px-3 py-1.5 h-full hover:bg-neutral-100">Mail</button>
		<Transition v-if="menuBarMenu=='app'" appear enter-active-class="transition duration-100 ease-linear" enter-to-class="translate-y-0 opacity-100" enter-from-class="-translate-y-1 opacity-0">
			<div class="absolute top-0 z-50 min-w-[8rem] text-neutral-800 rounded-md border border-neutral-200/70 bg-white mt-10 text-sm p-1 shadow-md w-48 -translate-x-0.5">
				<button @click="aboutMenu"  class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>About</span>
				</button>
				<button @click="closeMenu"  class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>Settings</span>
						<span class="ml-auto text-xs tracking-widest text-neutral-400 group-hover:text-neutral-600">⌘N</span>
				</button>
				<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
				<button @click="closeMenu"  class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>Quit</span>
				</button>
			</div>
		</Transition>
	</div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core'

export default {
	emits: ['close'],
	methods: {
		/**
		 * Close the menu
		 This is a custom event that is emitted to the parent component. Which is temporary and will be replaced with functions.
		 */
		closeMenu() {
			this.$emit('close')
		},
		/**
		 * Open the about menu
		 */
		aboutMenu() {
			this.$emit('close')
			invoke('create_about_window').then((message) => console.log(message))
		}
	},
	props: {
			menuBarOpen: Boolean,
			menuBarMenu: String
	}
}
</script>../../window/index.ts
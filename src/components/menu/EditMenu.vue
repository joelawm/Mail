<template>
	<div class="relative h-full cursor-default">
		<button class="rounded text-sm cursor-default flex items-center leading-tight justify-center px-3 py-1.5 h-full hover:bg-neutral-100">Edit</button>
		<Transition v-if="menuBarMenu=='edit'" enter-active-class="transition duration-100 ease-linear" enter-to-class="translate-y-0 opacity-100" enter-from-class="-translate-y-1 opacity-0">
				<div class="absolute top-0 z-50 min-w-[8rem] text-neutral-800 rounded-md border border-neutral-200/70 bg-white mt-10 text-sm p-1 shadow-md w-48 -translate-x-0.5">
					<button @click="closeMenu" class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
							<span>Undo</span>
							<span class="ml-auto text-xs tracking-widest text-neutral-400 group-hover:text-neutral-600">⌘Z</span>
					</button>
					<button @click="closeMenu" class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
							<span>Redo</span>
							<span class="ml-auto text-xs tracking-widest text-neutral-400 group-hover:text-neutral-600">⇧⌘Z</span>
					</button>
					<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
					<button class="relative w-full group">
							<div class="flex cursor-default select-none items-center rounded px-2 hover:bg-neutral-100 py-1.5 outline-none">
									<span>Find</span>
									<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 ml-auto"><polyline points="9 18 15 12 9 6"></polyline></svg>
							</div>
							<div data-submenu="" class="absolute top-0 right-0 invisible mr-1 duration-200 ease-out translate-x-full opacity-0 group-hover:mr-0 group-hover:visible group-hover:opacity-100">
									<div class="z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white p-1 shadow-md animate-in slide-in-from-left-1 w-32">
											<div @click="closeMenu" class="relative flex cursor-default select-none items-center rounded px-2 py-1.5 hover:bg-neutral-100 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50">Search the web</div>
											<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
											<div @click="closeMenu" class="relative flex cursor-default select-none items-center rounded px-2 py-1.5 hover:bg-neutral-100 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50">Find...</div>
											<div @click="closeMenu" class="relative flex cursor-default select-none items-center rounded px-2 py-1.5 hover:bg-neutral-100 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50">Find Next</div>
											<div @click="closeMenu" class="relative flex cursor-default select-none items-center rounded px-2 py-1.5 hover:bg-neutral-100 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50">Find Previous</div>
									</div>
							</div>
					</button>
					<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
					<button @click="closeMenu" class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
							<span>Cut</span>
					</button>
					<button @click="closeMenu" class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
							<span>Copy</span>
					</button>
					<button @click="closeMenu" class="relative flex justify-between w-full cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
							<span>Paste</span>
					</button>
			</div>
		</Transition>
	</div>
</template>

<script lang="ts">
export default {
	emits: ['close'],
	methods: {
		/**
		 * Close the menu
		 This is a custom event that is emitted to the parent component. Which is temporary and will be replaced with functions.
		 */
		closeMenu() {
			this.$emit('close')
		}
	},
	props: {
			menuBarOpen: Boolean,
			menuBarMenu: String
	}
}
</script>
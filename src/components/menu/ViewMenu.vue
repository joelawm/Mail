<template>
	<!-- View Button -->
	<div class="relative h-full cursor-default">
		<button @click="menuBarOpen=true; menuBarMenu='view'" @mouseover="menuBarMenu='view'" :class="{ 'bg-neutral-100' : menuBarOpen && menuBarMenu == 'view'}" class="rounded text-sm cursor-default flex items-center leading-tight justify-center px-3 py-1.5 h-full hover:bg-neutral-100">
				View
		</button>
		<div 
				x-show="menuBarOpen && menuBarMenu=='view'" 
				x-transition:enter="transition ease-linear duration-100" 
				x-transition:enter-start="-translate-y-1 opacity-90" 
				x-transition:enter-end="translate-y-0 opacity-100"
				class="absolute top-0 z-50 min-w-[15rem] text-neutral-800 rounded-md border border-neutral-200/70 bg-white mt-10 text-sm p-1 shadow-md w-48 -translate-x-0.5"
				x-cloak>

				<button @click="menuBarOpen=false; alwaysShowBookmarks=!alwaysShowBookmarks;" x-data="{ alwaysShowBookmarks: false }" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span x-show="alwaysShowBookmarks" class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4"><polyline points="20 6 9 17 4 12"></polyline></svg></span>
						<span>Always Show Bookmarks Bar</span>
				</button>
				<button @click="menuBarOpen=false; alwaysShowFullURL=!alwaysShowFullURL" x-data="{ alwaysShowFullURL: true }" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span x-show="alwaysShowFullURL" class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4"><polyline points="20 6 9 17 4 12"></polyline></svg></span>
						<span>Always Show Full URLs</span>
				</button>
				<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
				<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>Reload</span>
						<span class="ml-auto text-xs tracking-widest text-neutral-400 group-hover:text-neutral-600">⌘R</span>
				</button>
				<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none" data-disabled>
						<span>Force Reload</span>
						<span class="ml-auto text-xs tracking-widest text-neutral-400 group-hover:text-neutral-600">⇧⌘R</span>
				</button>
				<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
				<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>Toggle Fullscreen</span>
				</button>
				<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
				<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
						<span>Hide Sidebar</span>
				</button>
		</div>
	</div>
	<!-- End View Button -->
</template>
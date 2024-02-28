<template>
	<!-- Profiles Button -->
	<div class="relative h-full cursor-default">
		<button @click="menuBarOpen=true; menuBarMenu='profiles'" @mouseover="menuBarMenu='profiles'" :class="{ 'bg-neutral-100' : menuBarOpen && menuBarMenu == 'profiles'}" class="rounded text-sm cursor-default flex items-center leading-tight justify-center px-3 py-1.5 h-full hover:bg-neutral-100">
			Profiles
		</button>
		<div 
			x-show="menuBarOpen && menuBarMenu=='profiles'" 
			x-transition:enter="transition ease-linear duration-100" 
			x-transition:enter-start="-translate-y-1 opacity-90" 
			x-transition:enter-end="translate-y-0 opacity-100"
			class="absolute top-0 z-50 min-w-[8rem] text-neutral-800 rounded-md border border-neutral-200/70 bg-white mt-10 text-sm p-1 shadow-md w-48 -translate-x-0.5"
			x-cloak>

			<div class="relative w-full">
				<button @click="menuBarOpen=false" class="relative w-full flex cursor-default select-none items-center rounded py-1.5 pl-8 pr-2 hover:bg-neutral-100 outline-none data-[disabled]:opacity-50">
					<span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-2 h-2 fill-current"><circle cx="12" cy="12" r="10"></circle></svg></span>
					<span>Taylor Otwell</span>
				</button>
				<button @click="menuBarOpen=false" class="relative w-full flex cursor-default select-none items-center rounded py-1.5 pl-8 pr-2 hover:bg-neutral-100 outline-none data-[disabled]:opacity-50">
					<span>Adam Wathan</span>
				</button>
				<button @click="menuBarOpen=false" class="relative w-full flex cursor-default select-none items-center rounded py-1.5 pl-8 pr-2 hover:bg-neutral-100 outline-none data-[disabled]:opacity-50">
					<span>Caleb Porzio</span>
				</button>
			</div>
			<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
			<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
				<span>Edit...</span>
			</button>
			<div class="h-px my-1 -mx-1 bg-neutral-200"></div>
			<button @click="menuBarOpen=false" class="relative flex justify-between w-full pl-8 cursor-default select-none group items-center rounded px-2 py-1.5 hover:bg-neutral-100 hover:text-neutral-900 outline-none data-[disabled]:opacity-50 data-[disabled]:pointer-events-none">
				<span>Add Profile...</span>
			</button>
		</div>
	</div>
	<!-- End Profiles Button -->
</template>
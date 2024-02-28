<template>
	<div v-if="target!='macos'" v-click-away="closeMenu" class="fixed top-0 left-0 z-50 w-full transition-all duration-150 ease-out">
        <div class="relative top-0 left-0 z-40 w-auto h-10 transition duration-200 ease-out">
            <div class="w-full h-full bg-white border rounded-md border-neutral-200/80">
                <div class="flex w-full h-full select-none text-neutral-900">
                    <AppMenu @mouseover="hoverMenu('app')" @click="appMenu" @close="closeMenu" :menu-bar-menu="menuBarMenu" />
                    <FileMenu @mouseover="hoverMenu('file')" @click="fileMenu" @close="closeMenu" :menu-bar-menu="menuBarMenu"/>
                    <EditMenu @mouseover="hoverMenu('edit')" @click="editMenu" @close="closeMenu" :menu-bar-menu="menuBarMenu"/>
                </div>     
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core'
import AppMenu from "./AppMenu.vue";
import FileMenu from "./FileMenu.vue";
import EditMenu from "./EditMenu.vue";
import Package from '../../types/package';

export default {
    components: {
        AppMenu,
        FileMenu,
        EditMenu
    },
    async beforeCreate() {
        const msg = await invoke('package_version') as Package;
        console.log(msg)
        this.target = msg.target;
    },
    methods: {
        closeMenu() {
            this.menuBarOpen = false;
            this.menuBarMenu = '';
        },
        hoverMenu(item: string) {
            if (this.menuBarOpen == true) {
                this.menuBarMenu = item;
            }
        },
        appMenu() {
            this.menuBarOpen = true;
            this.menuBarMenu = 'app';
        },
        fileMenu() {
            this.menuBarOpen = true;
            this.menuBarMenu = 'file';
        },
        editMenu() {
            this.menuBarOpen = true;
            this.menuBarMenu = 'edit';
        }
    },
    data() {
        return {
            menuBarOpen: false,
            menuBarMenu: '',
            target: ''
        }
    }
}
</script>
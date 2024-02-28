<template>
	<div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
		<button @click="setAllInboxes" class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">All Inboxes</button>
		<div v-for="client in clients">
			<button @click="openDropdown(client.info.email)" class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
				<span class="flex-1 text-left rtl:text-right whitespace-nowrap">{{ client.info.email }}</span>
			</button>
			<ul v-if="openDropdowns.includes(client.info.email)" v-for="mailbox in client.mailbox" class="">
				<li>
					<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">{{ mailbox.mailbox_name }}</button>
				</li>
			</ul>
		</div>
	</div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Client, Mail } from '../../types/mail';

export default {
	name: 'Account',
	emits: ['letters'],
	props: {
		clients: {type: Array<Client>, required: true}
	},
	data() {
		return {
			openDropdowns: new Array<string>(),
		}
	},
	methods: {
		/**
		 * Grabs all inboxes from the Rust State.
		 */
		async setAllInboxes() {
			let data = await invoke('get_all_mailboxes') as [Mail];
			this.$emit('letters', data);
		},
		/**
		 * Grabs all mail from the selected mailbox.
		 */
		async selectMailbox(email: string, mailbox_name: string) {
			let data = await invoke('get_mailbox', {email, mailbox_name}) as [Mail];
			this.$emit('letters', data);
		},
		/**
		 * Opens and closes the dropdown.
		 */
		openDropdown(mailbox: string) {
      if (this.openDropdowns.includes(mailbox)) {
        const index = this.openDropdowns.indexOf(mailbox);
        this.openDropdowns.splice(index, 1);
      } else {
        this.openDropdowns.push(mailbox);
      }
    }
	},
}
</script>../../types/mail
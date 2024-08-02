<template>
	<div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
		<button @click="getAllInboxes" class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700 truncate">All Inboxes</button>
		<div v-for="client in clients">
			<button @click="openDropdown(client.info.email)" class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
				<chevronRightIcon v-if="!openDropdowns.includes(client.info.email)" class="w-5 h-5 mr-2 text-gray-900 dark:text-white" />
				<chevronDownIcon v-else class="w-5 h-5 mr-2 text-gray-900 dark:text-white" />
				<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{ client.info.email }}</span>
			</button>
			<ul v-if="openDropdowns.includes(client.info.email)" v-for="mailbox in client.mailbox" class="">
				<li>
					<div v-if="mailbox.mailbox_name == 'INBOX'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<MailIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-if="mailbox.mailbox_name == '[Gmail]/Sent Mail'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<SendIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else-if="mailbox.mailbox_name == '[Gmail]/Drafts'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<FileIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else-if="mailbox.mailbox_name == '[Gmail]/Trash'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<TrashIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else-if="mailbox.mailbox_name == '[Gmail]/Starred'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<StarIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else-if="mailbox.mailbox_name == '[Gmail]/All Mail'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<InboxIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else-if="mailbox.mailbox_name == '[Gmail]/Spam'">
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<ScissorIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{mailbox.mailbox_clean_name}}</span>
						</button>
					</div>
					<div v-else>
						<button @click="selectMailbox(client.info.email, mailbox.mailbox_name)" class="flex items-center w-full p-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">
							<FolderIcon class="w-5 h-5 mr-2 text-gray-900 dark:text-white"/>
							<span class="flex-1 text-left rtl:text-right whitespace-nowrap truncate">{{ mailbox.mailbox_clean_name }}</span>
						</button>
					</div>
				</li>
			</ul>
		</div>
	</div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Client, Mail } from '../../types/mail';
import chevronDownIcon from '../Icons/ChevronDownIcon.vue';
import chevronRightIcon from '../Icons/ChevronRightIcon.vue';
import InboxIcon from '../Icons/InboxIcon.vue';
import SendIcon from '../Icons/SendIcon.vue';
import FileIcon from '../Icons/FileIcon.vue';
import TrashIcon from '../Icons/TrashIcon.vue';
import StarIcon from '../Icons/StarIcon.vue';
import MailIcon from '../Icons/MailIcon.vue';
import ScissorIcon from "../Icons/ScissorIcon.vue";
import FolderIcon from "../Icons/FolderIcon.vue";

export default {
	name: 'Account',
	components: {
    chevronDownIcon,
    chevronRightIcon,
		InboxIcon,
		SendIcon,
		FileIcon,
		TrashIcon,
		StarIcon,
		MailIcon,
		ScissorIcon,
		FolderIcon
  },
	emits: ['letters'],
	props: {
		clients: {type: Array<Client>, required: true}
	},
	data() {
		return {
			openDropdowns: new Array<string>(),
			selectedMailbox: ''
		}
	},
	methods: {
		/**
		 * Grabs all inboxes from the Rust State.
		 */
		async getAllInboxes() {
			let data = await invoke('get_all_inboxes') as [Mail];
			this.$emit('letters', data);
		},
		/**
		 * Grabs all mail from the selected mailbox.
		 */
		async selectMailbox(email: string, mailbox: string) {
			let data = await invoke('get_mailbox', {email, mailbox}) as [Mail];
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
</script>
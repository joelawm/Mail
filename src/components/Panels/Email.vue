<template>
	<div v-for="mail in mailbox" @click="selectedLetter(mail.body.body_html)" style="border-bottom-width: 1px;"  className=" p-4 select-none border-gray-600 border-solid">
		<div class="flex items-center pr-2">
			<span v-if="!mail.flags.includes('\\Seen')" class="w-2 h-2 mr-2 bg-blue-200 rounded-full"></span>
			<h1 class="text-xl">{{ mail.from[0].name }}</h1>
		</div>
		<h2 class="text-gray-300">{{ mail.subject }}</h2>
		<div class="line-clamp-2 text-gray-500">{{ mail.body.body }}</div>
	</div>
</template>

<script lang="ts">
import Mail from '../../types/mail';

export default {
	name: 'Letter',
	emits: ['letter'],
	props: {
		mailbox: {type: Array<Mail>, required: true}
	},
	data() {
		return {
			mail: new Array<Mail>(),
		}
	},
	methods: {
		/**
		 * Grabs the selected letter from the Rust State.
		 */
		selectedLetter(letter: string) {
			this.$emit('letter', letter);
    },
	}
}

</script>
<template>
	<iframe className="flex-1" sandbox="allow-top-navigation-by-user-activation allow-popups-to-escape-sandbox" :srcdoc="email"></iframe>
</template>

<script lang="ts">
export default {
	name: 'Email',
	props: {
		email: {
			type: String,
			required: true
		}
	}
}
</script>

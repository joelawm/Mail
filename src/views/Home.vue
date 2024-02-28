<template>
	<Menu />
  <Multipane>
    <template v-slot:account>
      <Account @letters="(n) => mail = n" :clients="client"/>
    </template>
    <template v-slot:mail>
      <Email @letter="(n) => letter = n" :mailbox="mail" />
    </template>
    <template v-slot:letter>
      <Letter :email="letter" />
    </template>
  </Multipane>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Menu from '../components/menu/Menu.vue';
import Multipane from '../components/Multipane/Multipane.vue';
import Letter from '../components/Panels/Letter.vue';
import Account from '../components/Panels/Account.vue';
import Email from '../components/Panels/Email.vue';
import { Client, Mail } from '../types/mail';

export default {
  components: {
    Menu,
    Multipane,
    Letter,
    Email,
    Account
  },
  data() {
    return {
      client: new Array<Client>(),
      mail: new Array<Mail>(),
      letter: ''
    }
  },
  methods: {
    async refresh() {
      this.client = await invoke('client_connect') as [Client];
      this
      console.log(this.client);
    },
  },
  created() {
    this.refresh()
  }
}
</script>../types/mail
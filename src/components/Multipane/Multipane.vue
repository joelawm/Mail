<template>
  <div @mousemove="handleHover" @mouseleave="handleUp" @mouseup="handleUp" @mousedown="handleDown" :style="{cursor: cursor}" className="flex h-screen pt-10 text-white overflow-scroll">
    <div ref="account" :style="{width: account_max_width + 'px', minWidth: account_min_width + 'px'}" className="bg-gray-800 pr-1 select-none">
      <slot name="account" />
    </div>
    <div ref="mail" :style="{width: mail_max_width + 'px', minWidth: mail_min_width + 'px'}" className="flex flex-col pr-1 bg-gray-700 overflow-scroll">
      <slot name="mail" />
    </div>
    <div className="flex flex-1 flex-col min-w-52 bg-gray-600 overflow-scroll">
      <slot name="letter" />
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'Multipane',
  data() {
    return {
      onAccountBoarder: false,
      onMailBoarder: false,
      account_max_width: 250,
      mail_max_width: 500,
      account_min_width: 108,
      mail_min_width: 208,
      cursor: "auto"
    }
  },
  methods: {
    /**
     * This method is called when the mouse is moved over the box.
     * @param event 
     */
     handleHover(event: any) {
      const x = event.clientX;

      if (this.onAccountBoarder || this.onMailBoarder) {
        this.cursor = "col-resize";
      }
      else {
        const account = this.accountPos()
        const mail = this.mailPos();
        if (this.calculate(x, account.rectangle, account.borderSize)) {
          this.cursor = "col-resize";
        } else if (this.calculate(x, mail.rectangle, mail.borderSize)){
          this.cursor = "col-resize";
        } else {
          this.cursor = "auto";
        }
      }

      if ((this.account_max_width != x && x > this.account_min_width) && this.onAccountBoarder) {
        this.account_max_width = x;
      } else if ((this.mail_max_width - this.account_max_width != x && x > this.mail_min_width) && this.onMailBoarder) {
        this.mail_max_width = x - this.account_max_width;
      }
    },
    /**
     * This method is called when the box is clicked.
     * @param event 
     */
    handleDown(event: any) {
      const x = event.clientX;

      const account = this.accountPos();
      const mail = this.mailPos();

      if (this.calculate(x, account.rectangle, account.borderSize)) {
        this.onAccountBoarder = true;
      } else if (this.calculate(x, mail.rectangle, mail.borderSize)) {
        this.onMailBoarder = true;
      }
    },
    /**
     * This method is called when the mouse is moved away from the box.
     * @param event 
     */
    handleUp() {
      this.onAccountBoarder = false;
      this.onMailBoarder = false;
    },
    borderSize(box: any) {
      return parseInt(getComputedStyle(box).getPropertyValue('padding-right'));
    },
    /**
     * This method calculates wether were inside the box or not.
     * 
     * @param x 
     * @param y 
     * @param rect 
     * @param borderSize 
     */
    calculate(x: number,  rect: any, borderSize: number) {
      if ((x >= rect.x + rect.width - borderSize && x <= rect.x + rect.width)) {return true}
      else {return false}
    },
    accountPos() {
      const borderSize = this.borderSize(this.$refs.account);
      const rect = (this.$refs.account as HTMLDivElement).getBoundingClientRect();
      return {borderSize: borderSize, rectangle: rect}
    },
    mailPos() {
      const borderSize = this.borderSize(this.$refs.mail);
      const rect = (this.$refs.mail as HTMLDivElement).getBoundingClientRect();
      return {borderSize: borderSize, rectangle: rect}
    }
  },
}
</script>
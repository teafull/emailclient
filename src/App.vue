<script setup>
import MailClient from "./views/MailClient.vue";
</script>

<template>
  <MailClient />
</template>

<style>
:root {
  font-family: "Inter", "PingFang SC", "Microsoft YaHei", "Segoe UI", sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color: #1f2f44;
  background-color: #f2f5fb;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

body {
  margin: 0;
  background-color: #f2f5fb;
}

</style>

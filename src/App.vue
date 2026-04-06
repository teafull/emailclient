<script setup>
import { ref } from "vue";
import MailClient from "./views/MailClient.vue";
import LoginView from "./views/LoginView.vue";

const isLoggedIn = ref(false);

const handleLoginSuccess = () => {
  isLoggedIn.value = true;
};

const handleLogout = () => {
  isLoggedIn.value = false;
};
</script>

<template>
  <LoginView v-if="!isLoggedIn" @login-success="handleLoginSuccess" />
  <MailClient v-else @logout="handleLogout" />
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
  margin: 0;
  overflow: hidden;
}

body {
  background-color: #f2f5fb;
}
</style>

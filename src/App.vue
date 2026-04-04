<script setup>
import { ref, onMounted } from "vue";
import MailClient from "./views/MailClient.vue";
import LoginView from "./views/LoginView.vue";

const isLoggedIn = ref(false);

const readStorage = (key, fallback) => {
  const raw = localStorage.getItem(key);
  if (!raw) return fallback;
  try {
    return JSON.parse(raw);
  } catch {
    return fallback;
  }
};

onMounted(() => {
  const credentials = readStorage("mailCredentials", null);
  const settings = readStorage("mailSettings", {});
  if (credentials?.remember && credentials?.email && credentials?.password) {
    isLoggedIn.value = true;
  } else if (settings?.email && settings?.emailPassword) {
    isLoggedIn.value = true;
  }
});

const handleLoginSuccess = () => {
  isLoggedIn.value = true;
};

const handleLogout = () => {
  isLoggedIn.value = false;
  localStorage.removeItem("mailCredentials");
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

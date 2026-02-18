<script setup lang="ts">
import { watch, type Ref, ref } from "vue";
import Navbar from "./components/Navbar.vue";
import SoftBackdrop from "./components/SoftBackdrop.vue";

const isDark: Ref<boolean> = ref(localStorage.getItem("theme") === "dark");
watch(
  isDark,
  (val) => {
    document.documentElement.classList.toggle("dark", val);
    localStorage.setItem("theme", val ? "dark" : "light");
  },
  { immediate: true },
);

const toggleTheme = () => (isDark.value = !isDark.value);
// disable scroll always
// document.body.classList.add("overflow-hidden");
</script>

<template>
  <SoftBackdrop />

  <div
    class="min-h-screen bg-linear-to-b from-slate-100 to-white dark:from-gray-800 dark:to-gray-900 text-black dark:text-white transition-colors duration-300 flex flex-col"
  >
    <Navbar :isDark="isDark" @toggle-theme="toggleTheme" />
    <main class="mt-24">
      <router-view />
      </main>
  </div>
</template>

<style lang="css" scoped>
@import url("https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap");

body {
  font-family: "Inter", sans-serif;
}
</style>

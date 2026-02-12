<script setup lang="ts">
import { RouterLink } from "vue-router";
import ThemeButton from "@/components/ThemeButton.vue";
defineProps<{ isDark: boolean }>();
defineEmits(["toggle-theme"]);
</script>

<template>
  <nav class="fixed inset-x-0 top-5 z-10 px-4">
    <div
      class="max-w-6xl mx-auto flex items-center justify-between bg-white/40 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/50 rounded-2xl px-6 py-3 shadow-lg transition-all duration-300"
    >
      <RouterLink to="/">
        <h1
          class="text-xl font-bold bg-linear-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent hover:to-purple-700 dark:hover:from-blue-300 dark:hover:to-purple-300 transition-all"
        >
          Todo
        </h1>
      </RouterLink>
      <ThemeButton :is-dark="isDark" @toggle-theme="$emit('toggle-theme')" />
    </div>
  </nav>
</template>

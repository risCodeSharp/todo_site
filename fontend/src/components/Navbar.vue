<script setup lang="ts">
import ThemeButton from "./ThemeButton.vue";

defineProps<{ isDark: boolean }>();
defineEmits(["toggle-theme"]);
</script>

<template>
  <nav class="fixed inset-0 top-5 left-0 right-0 z-10 px-4">
    <div
      class="max-w-6xl mx-auto flex items-center justify-between rounded-2xl px-6 py-3 bg-white/40 dark:bg-slate-800/80 backdrop-blur-xl border border-slate-200/80 dark:border-slate-700/50 shadow-lg shadow-slate-200/50 dark:shadow-slate-900/50 transition-all duration-300"
    >
      <a href="#">
        <h1 class="text-xl font-bold bg-linear-to-r from-blue-600 to-purple-600 dark:to-purple-400 bg-clip-text text-transparent group-hover:to-purple-700 dark:group-hover:from-blue-300 dark:group-hover:to-purple-300 transition-all">Todo</h1>
      </a>
      <ThemeButton
        :is-dark="isDark"
        @toggle-theme="$emit('toggle-theme')"
      ></ThemeButton>
    </div>
  </nav>
</template>

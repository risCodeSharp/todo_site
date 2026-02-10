<script setup lang="ts">
import axios from "axios";
import { watch, onMounted, type Ref, ref } from "vue";
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


type Project = {
  id: number;
  name: string;
  tags: string[];
};

const projectList: Ref<Project[]> = ref([]);

const toggleTheme= () => isDark.value = !isDark.value;
onMounted(async () => {
  try {
    const response = await axios.get<Project[]>("/api/projects", {
      headers: { Accept: "application/json" },
    });

    projectList.value = response.data;
    console.log(projectList?.value, response.data);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.log("error: ", error.message);
    }
  }
});
</script>

<template>
  <SoftBackdrop />

  <div
  class="
  min-h-screen
  bg-linear-to-b from-slate-100 to-white
   dark:from-gray-800 dark:to-gray-900
   text-black dark:text-white 
   transition-colors duration-300 
   flex flex-col"
  >

    <Navbar :isDark="isDark" @toggle-theme="toggleTheme"/>
    
    <main class="mt-24 p-6 max-w-5xl mx-auto flex-1" >
      <h1 class="text-3xl font-bold text-violet-600 dark:text-violet-400">Todo Projects!</h1>
      <ul class="mt-6 space-y-2">
      <li
        v-for="project in projectList"
        :key="project.id"
        class="rounded-lg p-4
          bg-white/80 dark:bg-gray-800/80
          border border-gray-200 dark:border-gray-700
          shadow-sm
          hover:shadow-md transform hover:translate-y-8 transition duration-500
        "
      >     <button class="w-full text-left px-2 py-1 rounded transition-colors duration-200 hover:bg-violet-100 dark:hover:bg-violet-900/30">
          {{ project.name }}
        </button>
      </li>
      </ul>
      <button>add project</button>

  </main>
</div>
</template>

<style lang="css" scoped>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
body {
  font-family: 'Inter', sans-serif;
}
</style>

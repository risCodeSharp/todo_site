<script setup lang="ts">
import axios from "axios";
import { watch, onMounted, type Ref, ref } from "vue";
import Navbar from "./components/Navbar.vue";
import SoftBackdrop from "./components/SoftBackdrop.vue";
import { TrashIcon, ArrowRightIcon } from "@heroicons/vue/24/solid";

const isDark: Ref<boolean> = ref(localStorage.getItem("theme") === "dark");
const newProjectName: Ref<string> = ref("");
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
const fetchProjects = async () => {
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
};

const addProject = async () => {
  if (!newProjectName.value.trim()) return;

  try {
    await axios.post(
      "api/projects",
      { name: newProjectName.value },
      { headers: { Accept: "application/json" } },
    );

    newProjectName.value = "";
    await fetchProjects();
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.log("error: ", error.message);
    }
  }
};

const deleteProject = async (id: number) => {
  console.log("hello")
  try {
    await axios.delete(`api/projects/${id}`);

    projectList.value = projectList.value.filter((p) => p.id !== id);
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.log("error: ", error.message);
    }
  }
};

onMounted(fetchProjects);

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

    <main class="mt-24 p-6 max-w-5xl mx-auto flex-1">
      <h1 class="text-3xl font-bold text-violet-600 dark:text-violet-400">
        Todo Projects!
      </h1>

      <ul class="mt-6 space-y-2">
        <li
          v-for="project in projectList"
          :key="project.id"
          class="rounded-lg p-4 bg-white/90 dark:bg-gray-800/90 hover:bg-violet-50 dark:hover:bg-violet-900/30 border border-gray-200 dark:border-gray-700 hover:border-violet-300 dark:hover:border-violet-400 shadow-sm hover:shadow-lg transition-all duration-300 cursor-pointer"
        >
          <div class="flex gap-8 justify-between items-center">
            <span
              class="text-gray-800 dark:text-gray-200 hover:text-violet-700 dark:hover:text-violet-300 transition-colors font-medium"
            >
              {{ project.name }}
            </span>

            <button
              class="p-2.5 rounded-full bg-gray-100/50 hover:bg-red-500/20 dark:bg-gray-50/10 dark:hover:bg-violet-400/30 transition-all duration-200 hover:scale-110 active:scale-95 opacity-80 hover:opacity-100"
              aria-label="Delete Project" @click="deleteProject(project.id)"
            >
              <TrashIcon
                class="w-5 h-5 text-red-300 dark:text-gray-300 hover:text-red-600 dark:hover:text-red-100 transition-colors"
              />
            </button>
          </div>
        </li>
      </ul>
      <div class="my-3">
        <div class="flex gap-3 items-center">
          <label class="mb-6">
            Project Name
            <input
              class="bg-white/90 dark:bg-gray-800/90 shadow-2xl w-full border border-gray-400/15 rounded-lg p-4 hover:border-violet-300 dark:hover:border-violet-400 focus:outline-none focus:border-violet-200 focus:ring-2 focus:ring-violet-400/20 focus:bg-violet-50 dark:focus:bg-violet-900/30 dark:border-gray-700 transition-all duration-200"
              placeholder="add project here"
              v-model="newProjectName"
              @keyup.enter="addProject"
            />
          </label>
          <button
            class="flex-1 p-2.5 rounded-full h-10 bg-gray-100/50 hover:bg-violet-500/20 dark:bg-gray-50/10 dark:hover:bg-violet-400/30 transition-all duration-200 hover:scale-110 active:scale-95 opacity-80 hover:opacity-100"
            aria-label="Add Project"
            @click="addProject"
          >
            <ArrowRightIcon
              class="w-5 h-5 text-violet-300 dark:text-gray-100 hover:text-violet-600 dark:hover:text-violet-100 transition-colors"
            />
          </button>
        </div>
      </div>
    </main>
  </div>
</template>

<style lang="css" scoped>
@import url("https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap");

body {
  font-family: "Inter", sans-serif;
}
</style>

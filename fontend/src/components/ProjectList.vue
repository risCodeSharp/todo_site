<script lang="ts" setup>
import { TrashIcon} from "@heroicons/vue/24/solid";
import type { Project } from "@/types";
import { RouterLink } from "vue-router";
let { projectList } = defineProps<{
  projectList: Project[];
}>();

const emit = defineEmits<{
  (e: "delete", id: number): void;
}>();
</script>

<template>
  <ul class="mt-6 space-y-2">
    <li
      v-for="project in projectList"
      :key="project.id"
      class="rounded-lg p-4 bg-linear-to-r from-[#fdd9ec]/50 to-[#FF6B85]/50  dark:from-[#fdd9ec]/50 dark:to-[#FF6B85]/30 dark:bg-gray-800/90 hover:bg-violet-50 dark:hover:bg-violet-900/30 border border-gray-200 dark:border-gray-700 hover:border-violet-300 dark:hover:border-violet-400 shadow-sm hover:shadow-lg transition-all duration-300 cursor-pointer"
    >
    <div class="flex gap-8 justify-between items-center">
      <RouterLink :to="`/project-tasks/${project.id}`">
      <span
          class="text-gray-600 dark:text-gray-100 hover:text-violet-700 dark:hover:text-violet-300 transition-colors font-medium"
          >
          {{ project.name }}
        </span>
        
      </RouterLink>
        <button
        class="p-2.5 rounded-full bg-gray-100/50 hover:bg-red-500/20 dark:bg-gray-50/10 dark:hover:bg-violet-400/30 transition-all duration-200 hover:scale-110 active:scale-95 opacity-80 hover:opacity-100"
        aria-label="Delete Project"
        @click="emit('delete', project.id)"
        >
        <TrashIcon
        class="w-5 h-5 text-red-500 dark:text-gray-200 hover:text-red-600 dark:hover:text-red-100 transition-colors"
        />
      </button>
    </div>
    </li>
  </ul>
</template>

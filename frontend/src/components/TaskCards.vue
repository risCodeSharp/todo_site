<script lang="ts" setup>
import TaskCard from "@/components/TaskCard.vue";
import type { Task } from "@/types";

const props = defineProps<{ tasks: Task[] }>();

const emit = defineEmits<{
  (e: "reload-tasks"): void;
  (e: "delete-task", task_id: number): void;
}>();
</script>

<template>
  <div
    class="mx-auto w-full px-4 max-sm:w-11/12 sm:w-9/10 md:w-3/4 lg:w-4/5 xl:w-3/4 2xl:w-2/3 mb-6 columns-1 sm:columns-2 lg:columns-3 gap-4"
  >
    <div
      v-for="task in props.tasks"
      :key="task.id"
      class="mb-4 break-inside-avoid"
    >
      <TaskCard
        :task="task"
        @reload-tasks="emit('reload-tasks')"
        @delete-task="emit('delete-task', $event)"
      />
    </div>
  </div>
</template>

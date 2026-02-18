<script lang="ts" setup>
import { useRoute } from "vue-router";
import { onMounted, ref, type Ref } from "vue";
import axios from "axios";
import InputField from "@/components/InputField.vue";
import type { Project, Task } from "@/types";
import TaskCards from "@/components/TaskCards.vue";

const tasks: Ref<Task[]> = ref([]);
const projectName: Ref<string> = ref("");
const route = useRoute();
const projectId = route.params.id;
const newTaskName: Ref<string> = ref("");

// Fetch tasks from API
const fetchTasks = async () => {
  try {
    const response = await axios.get<Task[]>(
      `/api/projects/${projectId}/tasks`,
    );
    tasks.value = response.data;
    console.log("Tasks fetched:", tasks.value);
  } catch (error) {
    if (error instanceof Error) {
      console.log("Fetching tasks error:", error.message);
    }
  }
};

const fetchProjectName = async () => {
  try {
    const response = await axios.get<Project>(`/api/projects/${projectId}`);
    projectName.value = response.data.name;
  } catch (error) {
    if (error instanceof Error) {
      console.log("Fetching project_name error:", error.message);
    }
  }
}
const addTask = async () => {
  if (newTaskName.value.trim() === "") {
    return;
  }

  try {
    const response = await axios.post(
      `/api/projects/${projectId}/tasks`,
      {name: newTaskName.value},
      {headers: {Accept: "application/json"}},
    );
    await reloadTasks();
  } catch (error) {
    if (error instanceof Error) {
      console.log("Error Creating task:",error.message);
    }
  }
  newTaskName.value = "";
}

const deleteTask = (task_id: number) => {
  tasks.value = tasks.value.filter((task) => task.id !== task_id);
};


// Call fetchTasks on mount
onMounted(async () => {
  await Promise.all([fetchTasks(), fetchProjectName()]);
});

// Handler to reload tasks from child emit
const reloadTasks = () => {
  fetchTasks();
};
</script>

<template>
  <div class="block mx-auto w-70 md:w-100 mt-7">
    <h1 class="text-2xl font-bold text-violet-600 dark:text-violet-400">
      {{projectName}}
    </h1>

    <div class="mx-auto mb-10">
      <InputField
        class="block"
        label-text="Add Task"
        v-model="newTaskName"
        @on-submit="addTask()"
      />
    </div>
  </div>
  <TaskCards :tasks="tasks" @reload-tasks="reloadTasks" @delete-task="deleteTask"/>
</template>

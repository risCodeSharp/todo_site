<script lang="ts" setup>
import { useRoute } from "vue-router";
import { onMounted, ref, type Ref } from "vue";
import axios from "axios";
import type { Project, ProjectTasks, TaskItem } from "@/types";
import InputField from "@/components/InputField.vue";

const route = useRoute();
const projectId: number = Number(route.params.id);
const projectName: Ref<string> = ref("");
const tasks: Ref<TaskItem[]> = ref([]);

const newTaskName: Ref<string> = ref("hello");

const fetchProjectTitle = async () => {
  try {
    const response = await axios.get<Project>(`/api/projects/${projectId}`);
    const name = response.data.name;
    projectName.value =
      name.charAt(0).toUpperCase() + name.slice(1, name.length);
  } catch (error) {
    if (error instanceof Error) {
      console.log("Error: " + error.message);
    }
  }
};

const fetchProjectTaskList = async () => {
  try {
    const response = await axios.get<ProjectTasks>(
      `/api/projects/${projectId}/tasks`,
    );
    console.log("tasks: ", response.data);
    tasks.value = response.data.task_list;
  } catch (error) {
    if (error instanceof Error) {
      console.log("Error: " + error.message);
    }
  }
};

const createNewTask = async () => {
  try {
    await axios.post(
      `/api/projects/${projectId}`,
      { name: newTaskName.value },
      { headers: { Accept: "application/json" } },
    );
    await fetchProjectTaskList();
  } catch (error) {
    if (error instanceof Error) {
      console.log("error: ", error.message);
    }
  }
};

onMounted(async () =>
  Promise.all([fetchProjectTitle(), fetchProjectTaskList()]),
);
</script>

<template>
    <h1 class="text-2xl font-bold text-violet-600 dark:text-violet-400">
      {{ projectName }}
    </h1>
  <div>
    <InputField
      label-text="Add Task"
      v-model="newTaskName"
      @on-submit="createNewTask"
    />
    <div class="p-4">
      <ul v-for="task in tasks">
        <li>{{ task.id }}</li>
        <li>{{ task.name }}</li>
        <li>{{ task.is_completed }}</li>
      </ul>
    </div>
  </div>
</template>

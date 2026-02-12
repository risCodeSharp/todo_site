<script lang="ts" setup>
import { onMounted, type Ref, ref } from "vue";
import axios from "axios";
import ProjectList from "@/components/ProjectList.vue";
import InputField from "@/components/InputField.vue";
import type { Project } from "@/types";

const newProjectName: Ref<string> = ref("");
const projectList: Ref<Project[]> = ref([]);

const fetchProjects = async () => {
  try {
    const response = await axios.get<Project[]>("/api/projects", {
      headers: { Accept: "application/json" },
    });

    projectList.value = response?.data;
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
</script>
<template>
    <h1 class="text-3xl font-bold text-violet-600 dark:text-violet-400">
      Todo Projects!
    </h1>
    <InputField
      label-text="ProjectName"
      @on-submit="addProject"
      v-model="newProjectName"
    />
  <ProjectList :project-list="projectList" @delete="deleteProject" />
</template>

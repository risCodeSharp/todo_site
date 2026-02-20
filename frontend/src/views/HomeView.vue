<script lang="ts" setup>
import { onMounted, type Ref, ref } from "vue";
import axios from "axios";
import ProjectList from "@/components/ProjectList.vue";
import InputField from "@/components/InputField.vue";
import type { Project } from "@/types";
import Loading from "@/components/Loading.vue";

const newProjectName: Ref<string> = ref("");
const projectList: Ref<Project[]> = ref([]);
const isLoading: Ref<boolean> = ref(true);

const fetchProjects = async () => {
  isLoading.value = true;
  try {
    const response = await axios.get<Project[]>("/api/projects", {
      headers: { Accept: "application/json" },
    });

    projectList.value = response?.data;
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.log("error: ", error.message);
    }
  } finally {
    isLoading.value =  false;
  }
};

const addProject = async () => {
  if (!newProjectName.value.trim()) return;

  try {
    await axios.post(
      "/api/projects",
      { name: newProjectName.value, description: "" },
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
    await axios.delete(`/api/projects/${id}`);

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
    <div class="mt-24 p-10 max-w-5xl mx-auto flex-1">

    <h1 class="text-3xl font-bold text-violet-600 dark:text-violet-400">
      Todo Projects!
    </h1>
    <InputField
      label-text="Project Name"
      placeholder="project name"
      @on-submit="addProject"
      v-model="newProjectName"
    />
    <div v-if="isLoading" class="p-5 mt-4 flex items-center justify-center gap-3">
      <Loading /> 
      <p>Loading projects</p>
    </div>
  <ProjectList v-else :project-list="projectList" @delete="deleteProject" />
  </div>
</template>

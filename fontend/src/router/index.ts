import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";
import ProjectTasksView from "@/views/ProjectTasksView.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: HomeView,
  },
  {
    path: "/project-tasks/:id",
    name: "ProjectTasks",
    component: ProjectTasksView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

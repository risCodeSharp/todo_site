import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";
import TasksView from "@/views/TasksView.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: HomeView,
  },
  {
    path: "/project-tasks/:id",
    name: "ProjectTasks",
    component: TasksView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

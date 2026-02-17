<script lang="ts" setup>
import { onMounted, ref, type Ref } from "vue";
import Card from "./Card.vue";
import {
  PencilSquareIcon,
  CheckIcon,
  ChevronDownIcon,
  TrashIcon,
  PlusCircleIcon,
  XMarkIcon,
} from "@heroicons/vue/24/solid";
import type { Job, Task, WorkStatus } from "@/types";
import axios from "axios";

const { task } = defineProps<{ task: Task }>();
const emit = defineEmits<{
  (e: "reload-tasks"): void;
  (e: "delete-task", task_id: number): void;
}>();


const isAccordionOpen = ref(false);

const editingId = ref<number | null>(null);
const jobList: Ref<Job[]> = ref([]);
const newJob: Ref<string> = ref("");
// Helpers functions

const toggleEdit = (id: number) => {
  editingId.value = editingId.value === id ? null : id;
};

const isTaskEditing = ref(false);
const editableName = ref("");

const startTaskEditing = () => {
  editableName.value = task.name;
  isTaskEditing.value = true;
};

const saveTaskEditing = () => {
  task.name = editableName.value;
  isTaskEditing.value = false;
};

const cancelTaskEditing = () => {
  isTaskEditing.value = false;
};

const isEditing = (id: number) => editingId.value === id;

const statusColors: Record<WorkStatus, string> = {
  NotStarted: "text-gray-600 bg-gray-100",
  InProgress: "text-yellow-600 bg-yellow-100",
  Completed: "text-green-600 bg-green-100",
  Cancelled: "text-red-600 bg-red-100",
};

const getColor = (status: WorkStatus) =>
  statusColors[status] ?? "text-gray-600 bg-gray-100";

const formatStatus = (status: string) =>
  status.replace(/([A-Z])/g, " $1").trim();

// API Request and Response functions

const fetchJobs = async () => {
  try {
    const response = await axios.get<Job[]>(
      `/api/projects/${task.project_id}/tasks/${task.id}/jobs`,
    );
    jobList.value = response.data;
  } catch (error) {
    console.error("Fetch Jobs error:", error);
  }
};

const addJob = async () => {
  try {
    if (newJob.value.trim() === "") {
      return;
    }
    const response = await axios.post(
      `/api/projects/${task.project_id}/tasks/${task.id}/jobs`,
      { name: newJob.value },
    );
    newJob.value = "";
    fetchJobs();
    isAccordionOpen.value = true;
    
  } catch (error) {
    if (error instanceof Error) {
      console.log("Failed to add job: ", error.message);
    }
  }
};

const updateJob = async (job_id: number) => {
  try {
    const job = jobList.value.find((job) => job.id === job_id);
    const response = await axios.put(`/api/jobs/${job_id}`, {
      name: job?.name,
      difficulty: job?.difficulty,
      status: job?.status,
    });
    await fetchJobs();
    console.log(response.data);
  } catch (error) {
    if (error instanceof Error) {
      console.log("Failed to add job: ", error.message);
    }
  }
};

const handleEditClick = async (job_id: number) => {
  if (isEditing(job_id)) {
    await updateJob(job_id);
  }

  toggleEdit(job_id);
};

const deleteTask = async () => {
  try {
    await axios.delete(`/api/tasks/${task.id}`);
    emit('delete-task',task.id);
  } catch (error) {
    if (error instanceof Error) {
      console.log("Failed to add job: ", error.message);
    }
  }
};

const deleteJob = async (job_id: number) => {
  try {
    await axios.delete(`/api/jobs/${job_id}`);
    jobList.value = jobList.value.filter((job) => job.id !== job_id);
  } catch (error) {
    if (error instanceof Error) {
      console.log("Failed to add job: ", error.message);
    }
  }
};



onMounted(fetchJobs);
</script>

<template>
  <Card>
    <!-- Header -->
    <div
      class="bg-violet-50 dark:bg-violet-100 border-b border-gray-100 p-4 rounded-t-lg flex items-center justify-between"
    >
      <div class="flex-1">
        <h3 v-if="!isTaskEditing" class="text-xl font-semibold">
          {{ task.name }}
        </h3>

        <input
          v-else
          v-model="editableName"
          type="text"
          class="w-full px-2 py-1 rounded border border-violet-300 focus:outline-none focus:ring-2 focus:ring-violet-400"
        />
      </div>

      <!-- View Mode -->
      <div v-if="!isTaskEditing">
        <button
          @click="startTaskEditing"
          class="ml-4 p-2 bg-violet-500 text-white rounded hover:bg-violet-600 transition"
        >
          <PencilSquareIcon class="w-4 h-4" />
        </button>
        <button
          v-if="!isTaskEditing"
          @click="deleteTask()"
          class="ml-4 p-2 bg-violet-500 text-white rounded hover:bg-red-500 transition"
        >
          <TrashIcon class="w-4 h-4" />
        </button>
      </div>

      <!-- Edit Mode -->
      <div v-else class="ml-4 flex items-center gap-2">
        <button
          @click="saveTaskEditing"
          class="p-2 bg-violet-500 text-white rounded hover:bg-violet-600 transition"
        >
          <CheckIcon class="w-4 h-4" />
        </button>

        <button
          @click="cancelTaskEditing"
          class="p-2 border border-violet-400 text-violet-700 rounded hover:bg-violet-100 transition"
        >
          <XMarkIcon class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Add Job Input -->
    <div class="pt-4 w-full">
      <div
        class="flex items-center h-12 w-full max-w-md bg-white border border-gray-300 rounded mb-4"
      >
        <button class="h-full px-3 hover:bg-violet-50 shrink-0">
          <PlusCircleIcon class="w-5 h-5 text-violet-500" />
        </button>
        <input
          class="flex-1 min-w-0 outline-none px-2"
          placeholder="New job..."
          @keypress.enter="addJob"
          v-model="newJob"
        />
        <button
          class="h-full px-4 shrink-0 text-violet-600 font-medium hover:bg-violet-50"
          @click="addJob"
        >
          Add
        </button>
      </div>
    </div>

    <!-- Accordion -->
    <div class="relative">
      <button
        @click="isAccordionOpen = !isAccordionOpen"
        class="flex justify-center items-center w-8 h-8 rounded-full mx-auto mb-3 bg-violet-800 text-white hover:bg-violet-900 shadow-sm transition-all"
      >
        <ChevronDownIcon
          class="w-5 h-5 transition-transform duration-300"
          :class="{ 'rotate-180': isAccordionOpen }"
        />
      </button>

      <div
        class="overflow-hidden transition-all duration-500"
        :class="
          isAccordionOpen ? 'max-h-[2000px] opacity-100' : 'max-h-0 opacity-0'
        "
      >
        <ul
          class="bg-white shadow overflow-hidden sm:rounded-md max-w-md mx-auto"
        >
          <li
            v-for="(job, index) in jobList"
            :key="job.id"
            :class="{ 'border-t border-gray-200': index > 0 }"
          >
            <div class="px-4 py-5">
              <!-- Job Header -->
              <div class="flex items-center justify-between">
                <h3 class="text-lg font-medium text-gray-900">
                  {{ job.name }}
                </h3>
                <span
                  class="px-2 py-1 rounded-full text-xs font-semibold"
                  :class="getColor(job.status)"
                >
                  {{ formatStatus(job.status) }}
                </span>
              </div>

              <!-- Difficulty -->
              <p v-if="job.difficulty" class="mt-2 text-sm text-gray-500">
                Difficulty:
                <span class="text-violet-600 font-medium">{{
                  job.difficulty
                }}</span>
              </p>

              <!-- Edit Button -->
              <div class="mt-4 flex justify-end gap-1">
                <button
                  @click="handleEditClick(job.id)"
                  class="text-sm flex items-center gap-1 transition-all p-1 rounded-md p-1"
                  :class="
                    isEditing(job.id)
                      ? 'text-green-600 hover:bg-green-100'
                      : 'text-indigo-600 hover:bg-violet-100'
                  "
                >
                  <span>{{ isEditing(job.id) ? "Done" : "Edit" }}</span>
                  <CheckIcon v-if="isEditing(job.id)" class="w-4 h-4" />
                  <PencilSquareIcon v-else class="w-4 h-4" />
                </button>
                <button
                  @click="deleteJob(job.id)"
                  class="text-sm flex items-center transition-all text-red-600 p-1 rounded-md hover:bg-red-100"
                  v-if="!isEditing(job.id)"
                >
                  <span>Delete</span>
                  <TrashIcon class="w-4 h-4" />
                </button>
              </div>

              <!-- Edit Section -->
              <div
                class="overflow-hidden transition-all duration-500"
                :class="
                  isEditing(job.id)
                    ? 'max-h-[500px] opacity-100 mt-4'
                    : 'max-h-0 opacity-0'
                "
              >
                <div class="space-y-3">
                  <!-- Editable Job Name -->
                  <div>
                    <label class="text-xs font-semibold text-gray-500"
                      >Job Title:</label
                    >
                    <input
                      type="text"
                      v-model="job.name"
                      class="w-full mt-1 text-sm border border-gray-200 rounded-lg p-2 focus:ring-2 focus:ring-violet-500"
                    />
                  </div>

                  <!-- Status Selector -->
                  <div>
                    <label class="text-xs font-semibold text-gray-500"
                      >Status:</label
                    >
                    <select
                      v-model="job.status"
                      class="w-full mt-1 text-sm border border-gray-200 rounded-lg p-2 focus:ring-2 focus:ring-violet-500"
                    >
                      <option value="NotStarted">Not Started</option>
                      <option value="InProgress">In Progress</option>
                      <option value="Completed">Completed</option>
                      <option value="Cancelled">Cancelled</option>
                    </select>
                  </div>

                  <!-- Difficulty Selector -->
                  <div>
                    <label class="text-xs font-semibold text-gray-500"
                      >Difficulty:</label
                    >
                    <select
                      v-model="job.difficulty"
                      class="w-full mt-1 text-sm border border-gray-200 rounded-lg p-2 focus:ring-2 focus:ring-violet-500"
                    >
                      <option>Easy</option>
                      <option>Medium</option>
                      <option>Hard</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </li>

          <!-- Empty State -->
          <li v-if="jobList.length === 0" class="p-6 text-center text-gray-400">
            No jobs yet.
          </li>
        </ul>
      </div>
    </div>
  </Card>
</template>

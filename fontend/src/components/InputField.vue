<script lang="ts" setup>
import { ArrowRightIcon } from "@heroicons/vue/24/solid";

defineProps<{ labelText: string; modelValue: string }>();
defineEmits<{
  (e: "onSubmit"): void;
  (e: "update:modelValue", value: string): void;
}>();
</script>

<template>
  <div class="m-1">
    <label
      class="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
    >
      {{ labelText }}
    </label>
    <div class="flex gap-2 items-center">
      <input
        class="flex-1  bg-white dark:bg-gray-800/90 shadow-2xl 
        border border-gray-400/15 rounded-lg
         px-4 py-3 
         hover:border-violet-300 dark:hover:border-violet-400
          focus:outline-none focus:border-violet-200 
          focus:ring-2 focus:ring-violet-400/20 
          focus:bg-violet-50 
          dark:focus:bg-violet-900/30 dark:border-gray-700 
          transition-all duration-200 
          placeholder:text-gray-400 dark:placeholder:text-gray-500"
        placeholder="add project here"
        :value="modelValue"
        @input="
          $emit('update:modelValue', ($event.target as HTMLInputElement).value)
        "
        @keyup.enter="$emit('onSubmit')"
      />
      <button
        class="flex items-center justify-center w-11 h-11 rounded-full bg-gray-100/50 hover:bg-violet-500/20 dark:bg-gray-50/10 dark:hover:bg-violet-400/30 transition-all duration-200 hover:scale-110 active:scale-95 opacity-80 hover:opacity-100 flex-shrink-0"
        aria-label="Add Project"
        @click="$emit('onSubmit')"
      >
        <ArrowRightIcon
          class="w-5 h-5 text-violet-300 dark:text-gray-100 hover:text-white"
        />
      </button>
    </div>
  </div>
</template>

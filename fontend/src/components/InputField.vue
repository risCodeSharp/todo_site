<script lang="ts" setup>
import { ArrowRightIcon } from "@heroicons/vue/24/solid";

defineProps<{ labelText: string; modelValue: string }>();
defineEmits<{
  (e: "onSubmit"): void;
  (e: "update:modelValue", value: string): void;
}>();
</script>

<template>
  <div class="my-3">
    <div class="flex gap-3 items-center">
      <label class="block mb-2 text-sm font-medu">
        {{ labelText }}
        <input
          class="bg-white/90 dark:bg-gray-800/90 shadow-2xl w-full border border-gray-400/15 rounded-lg p-4 hover:border-violet-300 dark:hover:border-violet-400 focus:outline-none focus:border-violet-200 focus:ring-2 focus:ring-violet-400/20 focus:bg-violet-50 dark:focus:bg-violet-900/30 dark:border-gray-700 transition-all duration-200"
          placeholder="add project here"
          :value="modelValue"
          @input="
            $emit(
              'update:modelValue',
              ($event.target as HTMLInputElement).value,
            )
          "
          @keyup.enter="$emit('onSubmit')"
        />
      </label>
      <button
        class="flex-1 p-2.5 rounded-full h-10 bg-gray-100/50 hover:bg-violet-500/20 dark:bg-gray-50/10 dark:hover:bg-violet-400/30 transition-all duration-200 hover:scale-110 active:scale-95 opacity-80 hover:opacity-100"
        aria-label="Add Project"
        @click="$emit('onSubmit')"
      >
        <ArrowRightIcon
          class="w-5 h-5 text-violet-300 dark:text-gray-100 hover:text-violet-600 dark:hover:text-violet-100 transition-colors"
        />
      </button>
    </div>
  </div>
</template>

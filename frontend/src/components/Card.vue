<template>
  <div
    class="max-sm:w-1/1 md:1/2 lg:1/3 lg:1/4
           dark: bg-linear-to-br
           dark:from-purple-500/50
           dark:via-violet-500/25
           dark:to-indigo-500/5
           m-1 
           bg-linear-to-b
         from-white
         to-gray-100/30
           backdrop-blur-xl
           border border-white/10
           rounded-2xl
           shadow-lg
           hover:shadow-[0_0_35px_rgba(139,92,246,0.25)]
           
           transition-all duration-500
           p-5  text-violet-900 dark:text-purple-700"
           
  >
    <slot></slot>
  </div>
</template>

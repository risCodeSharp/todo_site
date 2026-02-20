<template>
  <!-- <div class="flex items-center justify-center min-h-screen bg-black"> -->
    <div class="relative">
      <div class="relative w-8 h-8">
        <!-- Outer Ring -->
        <div
          class="absolute w-full h-full rounded-full border-[3px] border-gray-100/10 border-r-[#8B5CF6] border-b-[#8B5CF6] animate-spin"
          style="animation-duration: 3s;"
        ></div>

        <!-- Inner Ring -->
        <div
          class="absolute w-full h-full rounded-full border-[3px] border-gray-100/10 border-t-[#8B5CF6] animate-spin"
          style="animation-duration: 2s; animation-direction: reverse;"
        ></div>
      </div>

      <!-- Glow Effect -->
      <div
        class="absolute inset-0 bg-linear-to-tr from-[#8B5CF6]/20 via-transparent to-[#8B5CF6]/10 animate-pulse rounded-full blur-sm"
      ></div>
    </div>
  <!-- </div> -->
</template>
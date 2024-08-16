<template>
  <div class="px-4 py-12 flex flex-col gap-8">
    <h1 class="font-bold leading-[22px] text-center text-[18px]">Welcome!</h1>
    <div class="flex justify-center items-center">
      <img
        src="../../assets/welcome-page-illustration.png"
        alt=""
        class="p-4 mt-12 w-[180px]"
      />
    </div>
    <p class="leading-8 text-center p-[14px]">
      Increase your productivity and manage your time effectively with Pomodore.
    </p>
    <div class="flex flex-col gap-8">
      <RouterLink 
       :to="{ name: 'sign-up' }"
        class="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-[#f5f5f5]"
      >
        Sign up
      </RouterLink>

      <RouterLink
        :to="{ name: 'login' }"
        class="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-[#f5f5f5] bg-[#75CE8E]"
      >
        Sign In
      </RouterLink>

      <button
        class="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-white flex items-center justify-center gap-2"
      >
        <img src="../../assets/google-icon.png" alt="" class="w-[24px]" />
        <span> Continue with Google</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

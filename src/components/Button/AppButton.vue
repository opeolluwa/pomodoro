<template>
  <button
    :class="[
      'shadow rounded-[5px] py-[15px] px-[20px] text-center',
      backgroundColor,
      textColor,
      fontWeight,
    ]"
  >
    {{ text }} 
  </button>
</template>

<script lang="ts" setup>
const props = defineProps({
  text: String,
  bg: String,
  color: String,
  weight: String,
});
const backgroundColor = "bg-" + props.bg;
const textColor = "text-" + props.color;
const fontWeight = "font-" + props.weight;
</script>

<style></style>

<script setup lang="ts">
type ToDo = {
  id: number
  title: string
  completed: boolean
}
const list: ToDo[] = [
  {
    id: 1,
    title: 'Learn Vue 3',
    completed: false
  },
  {
    id: 2,
    title: 'Learn TypeScript',
    completed: false
  },
  {
    id: 3,
    title: 'Learn Tailwind CSS',
    completed: true
  }
]
</script>

<template>
  <main class="container text-red-500 mt-4">
    <li v-for="item in list" :key="item.id">
      {{ item.title }}
    </li>
  </main>
</template>

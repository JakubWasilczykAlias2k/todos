<script setup lang="ts">
import { ref } from 'vue'

type ToDo = {
  id: number
  title: string
  completed: boolean
}
const list = ref<ToDo[]>([
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
])
const newItemTitle = ref('')
function addOne() {
  list.value.push({
    id: list.value.length + 1,
    title: newItemTitle.value,
    completed: false
  })
}
</script>

<template>
  <main class="container mt-4">
    <form class="my-4 flex gap-2" @submit.prevent="addOne">
      <input id="title" name="title" type="text" v-model="newItemTitle" placeholder="Add item" />
      <button class="bg-slate-500 rounded px-3 py-2 text-slate-50" type="submit">Add</button>
    </form>
    <li v-for="item in list" :key="item.id">
      {{ item.title }}
    </li>
  </main>
</template>

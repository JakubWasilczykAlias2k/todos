<script setup lang="ts">
import { ref } from 'vue'

interface ToDo {
  id: number
  title: string
  description: string
  completed: boolean
}
const list = ref<ToDo[]>([])

async function fetchList() {
  const response = await fetch('http://localhost:6969/todos', { method: 'GET' })
  const json = await response.json()
  list.value = json
}
fetchList()
</script>

<template>
  <main class="container mt-4">
    <li v-for="item in list" :key="item.id">
      {{ item.title }}
    </li>
  </main>
</template>

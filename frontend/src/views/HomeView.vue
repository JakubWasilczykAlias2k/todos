<script setup lang="ts">
import { ref } from 'vue'
import TodoForm from '@/components/TodoForm.vue'

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
    <TodoForm />
    <li v-for="item in list" :key="item.id">
      {{ item.title }}
    </li>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import TodoForm from '@/components/TodoForm.vue'
import { Button } from '@/components/ui/button'

interface ToDo {
  id: number
  title: string
  description: string
  completed: boolean
}
const list = ref<ToDo[]>([])
const isOpen = ref(false)

async function fetchList() {
  const response = await fetch('http://localhost:6969/todos', { method: 'GET' })
  const json = await response.json()
  list.value = json
}
fetchList()
</script>

<template>
  <main class="container mt-4">
    <Button @click="isOpen = true">Create new To Do</Button>
    <TodoForm :open="isOpen" @update:open="(v) => (isOpen = v)" />

    <li v-for="item in list" :key="item.id">
      {{ item.title }}
    </li>
  </main>
</template>

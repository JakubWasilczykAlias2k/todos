<script setup lang="ts">
import { ref } from 'vue'
import TodoForm from '@/components/TodoForm.vue'
import TodosTable from '@/components/TodosTable.vue'
import { Button } from '@/components/ui/button'
import type { ToDo } from '@/models/todo'

const list = ref<ToDo[]>([])
const isOpen = ref(false)

async function fetchList() {
  const response = await fetch('http://localhost:6969/todos', { method: 'GET' })
  const json = await response.json()
  list.value = json
}
fetchList()

function onSaved() {
  isOpen.value = false
  fetchList()
}
</script>

<template>
  <main class="container mt-4">
    <Button @click="isOpen = true">Create new To Do</Button>
    <TodoForm :open="isOpen" @update:open="(v) => (isOpen = v)" @saved="onSaved" />
    <section class="py-6">
      <TodosTable :list="list" @list:changed="fetchList" />
    </section>
  </main>
</template>

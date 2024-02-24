<script setup lang="ts">
import type { ToDo } from '@/models/todo'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { CheckCircleIcon, CircleIcon } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'

const { params } = useRoute()
const id = params.id as string

const todo = ref<ToDo>()
async function fetchItem() {
  const response = await fetch(`http://localhost:6969/todos/${id}`, { method: 'GET' })
  const json = await response.json()
  todo.value = json
}
fetchItem()

async function toggleComplete(id: number, value: boolean) {
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'PATCH',
    body: JSON.stringify({
      completed: value
    }),
    headers: {
      'Content-Type': 'application/json'
    }
  })
  fetchItem()
}
</script>

<template>
  <main v-if="todo">
    <section class="relative h-[26rem]">
      <img
        class="object-cover absolute inset-0 w-full h-full"
        :src="`https://picsum.photos/seed/montevarchi-${todo.id}/1920/416`"
      />
      <div
        class="absolute inset-x-0 bottom-0 top-1/2 py-4 text-white bg-gradient-to-t to-transparent from-black/80"
      >
        <div class="container flex justify-between items-end h-full">
          <h1 class="text-5xl font-bold">{{ todo.title }}</h1>
          <Button
            v-if="todo.completed"
            variant="ghost"
            class="flex gap-2 items-center text-lg font-bold text-green-400"
            @click="toggleComplete(todo.id, false)"
          >
            <CheckCircleIcon stroke-width="3" class="w-6 h-6" /> Completed
          </Button>
          <Button
            v-else
            variant="ghost"
            class="flex gap-2 items-center text-lg font-bold text-neutral-400"
            @click="toggleComplete(todo.id, true)"
          >
            <CircleIcon stroke-width="3" class="w-6 h-6" /> Not Completed
          </Button>
        </div>
      </div>
    </section>
    <section class="container py-2">
      <p class="text-lg font-light">{{ todo.description }}</p>
    </section>
  </main>
</template>

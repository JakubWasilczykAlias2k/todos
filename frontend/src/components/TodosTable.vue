<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from '@/components/ui/table'
import EditTodoView from '@/components/EditTodoView.vue'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Trash2Icon, PencilIcon } from 'lucide-vue-next'
import type { ToDo } from '@/models/todo'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

type Props = {
  list: ToDo[]
}
type Emits = {
  (e: 'list:changed'): void
}
const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const editTodoOpen = ref<boolean>(false)
const editTodo = ref<ToDo>()

function onEditClick(item: ToDo) {
  editTodoOpen.value = true
  editTodo.value = item
}

async function deleteToDo(id: number) {
  await fetch(`http://localhost:6969/todos/${id}`, {
    method: 'DELETE'
  })
  emits('list:changed')
}

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
  emits('list:changed')
}

function onEditSaved() {
  editTodoOpen.value = false
  emits('list:changed')
}

const { push } = useRouter()
</script>

<template>
  <Table>
    <TableCaption>List of things to do</TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead class="w-1/12"></TableHead>
        <TableHead class="w-4/12"> Title </TableHead>
        <TableHead class="w-5/12"> Description </TableHead>
        <TableHead class="w-2/12"></TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="item in list" class="group" :key="`todo-${item.id}`">
        <TableCell>
          <Checkbox @click="toggleComplete(item.id, !item.completed)" :checked="item.completed" />
        </TableCell>
        <TableCell @click="push(`/todos/${item.id}`)" class="cursor-pointer">
          {{ item.title }}
        </TableCell>
        <TableCell @click="push(`/todos/${item.id}`)" class="cursor-pointer">
          {{ item.description }}
        </TableCell>
        <TableCell
          class="opacity-0 transition-opacity duration-500 group-hover:opacity-100 text-end"
        >
          <Button variant="ghost" @click="deleteToDo(item.id)" size="icon">
            <Trash2Icon class="text-red-500" />
          </Button>
          <Button variant="ghost" @click="onEditClick(item)" size="icon">
            <PencilIcon />
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
  <EditTodoView
    v-if="editTodo"
    :open="editTodoOpen"
    :todo="editTodo"
    @update:open="(v) => (editTodoOpen = v)"
    @saved="onEditSaved"
  />
</template>

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
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Trash2Icon } from 'lucide-vue-next'
import type { ToDo } from '@/models/todo'

type Props = {
  list: ToDo[]
}
type Emits = {
  (e: 'list:changed'): void
}
const props = defineProps<Props>()
const emits = defineEmits<Emits>()

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
</script>

<template>
  <Table>
    <TableCaption>List of things to do</TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead></TableHead>
        <TableHead> Title </TableHead>
        <TableHead> Description </TableHead>
        <TableHead></TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="item in list" class="group" :key="`todo-${item.id}`">
        <TableCell>
          <Checkbox @click="toggleComplete(item.id, !item.completed)" :checked="item.completed" />
        </TableCell>
        <TableCell>{{ item.title }}</TableCell>
        <TableCell>{{ item.description }}</TableCell>
        <TableCell class="opacity-0 transition-opacity duration-500 group-hover:opacity-100">
          <Button variant="ghost" size="icon">
            <Trash2Icon class="text-red-500" />
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>

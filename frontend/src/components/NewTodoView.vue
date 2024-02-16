<script setup lang="ts">
import { TODO_SCHEMA } from './TodoForm.vue'
import { toTypedSchema } from '@vee-validate/zod'
import TodoForm from './TodoForm.vue'
import { Button } from '@/components/ui/button'
import { Form } from '@/components/ui/form'
import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogContent,
  DialogFooter
} from '@/components/ui/dialog'

type Props = {
  open: boolean
}
type Emits = {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}
const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const schema = toTypedSchema(TODO_SCHEMA)

async function onSubmit(values: any) {
  const body = {
    ...values,
    completed: false
  }
  await fetch('http://localhost:6969/todos', {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json'
    }
  })
  emits('saved')
}
</script>

<template>
  <Dialog :open="open" @update:open="(v) => emits('update:open', v)">
    <DialogContent>
      <DialogHeader>
        <DialogTitle> New To Do </DialogTitle>
        <DialogDescription> Create a new To Do item </DialogDescription>
      </DialogHeader>

      <Form
        class="flex flex-col gap-8 items-start my-3"
        @submit="onSubmit"
        :validation-schema="schema"
      >
        <TodoForm />

        <DialogFooter>
          <Button @click="emits('update:open', false)" variant="ghost" type="button">
            Cancel
          </Button>
          <Button type="submit"> Save </Button>
        </DialogFooter>
      </Form>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import * as z from 'zod'
import { toTypedSchema } from '@vee-validate/zod'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter
} from '@/components/ui/dialog'
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
  FormDescription
} from '@/components/ui/form'

type Props = {
  open: boolean
}
type Emits = {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}
const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const TODO_SCHEMA = z.object({
  title: z.string().min(3).max(100),
  description: z.string().max(1000).default('')
})
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
      <Form
        class="flex flex-col gap-8 items-start my-3"
        @submit="onSubmit"
        :validation-schema="schema"
      >
        <DialogHeader>
          <DialogTitle> New To Do </DialogTitle>
          <DialogDescription> Create a new To Do item </DialogDescription>
        </DialogHeader>
        <FormField name="title" v-slot="{ componentField }">
          <FormItem class="w-full">
            <FormLabel>Title</FormLabel>
            <FormDescription>The title of new ToDo</FormDescription>
            <FormControl>
              <Input class="w-full" placeholder="Get the groceries" v-bind="componentField" />
            </FormControl>
            <FormMessage />
          </FormItem>
        </FormField>
        <FormField name="descripton" v-slot="{ componentField }">
          <FormItem class="w-full">
            <FormLabel>Description</FormLabel>
            <FormDescription>The description of this ToDo</FormDescription>
            <FormControl>
              <Input class="w-full" v-bind="componentField" />
            </FormControl>
            <FormMessage />
          </FormItem>
        </FormField>
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

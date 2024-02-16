<script lang="ts">
import z from 'zod'

export const TODO_SCHEMA = z.object({
  title: z.string().min(3).max(100),
  description: z.string().max(1000).default('')
})

export type ToDoSchema = z.infer<typeof TODO_SCHEMA>
</script>

<script setup lang="ts">
import { Input } from '@/components/ui/input'
import {
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormMessage,
  FormDescription
} from '@/components/ui/form'
</script>

<template>
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
  <FormField name="description" v-slot="{ componentField }">
    <FormItem class="w-full">
      <FormLabel>Description</FormLabel>
      <FormDescription>The description of this ToDo</FormDescription>
      <FormControl>
        <Input class="w-full" v-bind="componentField" />
      </FormControl>
      <FormMessage />
    </FormItem>
  </FormField>
</template>

<script setup lang="ts">
import { Form as VeeForm, Field as VeeField, ErrorMessage as VeeErrorMessage } from 'vee-validate'
import * as z from 'zod'
import { toTypedSchema } from '@vee-validate/zod'

const TODO_SCHEMA = z.object({
  title: z.string().min(3).max(100),
  description: z.string().max(1000).default('')
})
const schema = toTypedSchema(TODO_SCHEMA)

async function onSubmit(values: any) {
  console.log(values)
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
  window.location.reload()
}
</script>

<template>
  <VeeForm
    class="flex flex-col gap-2 items-start my-3 mx-auto max-w-md"
    @submit="onSubmit"
    :validation-schema="schema"
  >
    <label class="flex flex-col gap-1 w-full">
      Title
      <VeeField
        class="py-2 px-3 rounded border bg-slate-50 border-slate-300"
        name="title"
        placeholder="Title"
      />
      <VeeErrorMessage name="title" class="text-red-500" />
    </label>
    <label class="flex flex-col gap-1 w-full">
      Description
      <VeeField
        class="py-2 px-3 rounded border bg-slate-50 border-slate-300"
        name="description"
        placeholder="Description"
      />
      <VeeErrorMessage name="description" class="text-red-500" />
    </label>
    <button class="py-2 px-3 rounded bg-slate-500 text-slate-50">Submit</button>
  </VeeForm>
</template>

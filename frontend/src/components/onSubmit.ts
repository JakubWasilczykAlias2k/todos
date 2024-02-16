import { emits } from './NewTodoView.vue'

export async function onSubmit(values: ToDoSchema) {
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

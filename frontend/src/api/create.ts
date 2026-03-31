import { TodoItems, TodoItem, TaskStatus } from "../interfaces/toDoItems";
import { postCall } from "./utils";
import { Url } from "./url";

export async function createToDoItemCall(title: string) {
  const toDoItem: TodoItem = {
    title: title,
    status: TaskStatus.PENDING,
  };
  return postCall<TodoItem, TodoItems>(new Url().create, toDoItem, 201);
}

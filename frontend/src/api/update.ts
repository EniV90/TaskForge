import { TaskStatus, TodoItem, TodoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { patchCall } from "./utils";

export async function updateToDoItemCall(name: string, status: TaskStatus) {
  const toDoItem: TodoItem = {
    title: name,
    status: status,
  };
  return patchCall<TodoItem, TodoItems>(new Url().update, toDoItem, 200);
}

import { TaskStatus, ToDoItem, ToDoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { patchCall } from "./utils";

export async function updateToDoItemCall(
  name: string,
  status: TaskStatus,
  id: number
) {
  const toDoItem: ToDoItem = {
    title: name,
    status: status,
    id: id,
  };
  return patchCall<ToDoItem, ToDoItems>(new Url().update, toDoItem, 200);
}

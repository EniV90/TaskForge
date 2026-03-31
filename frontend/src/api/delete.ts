import { TodoItem, TodoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { deleteCall } from "./utils";

export async function deleteToDoItemCall(name: string) {
  return deleteCall<TodoItems>(new Url().deleteUrl(name), 200);
}

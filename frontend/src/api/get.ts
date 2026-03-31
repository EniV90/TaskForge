import { TodoItems } from "../interfaces/toDoItems";
import { getCall } from "./utils";
import { Url } from "./url";

export default async function getAll() {
  let response = await getCall<TodoItems>(new Url().getAll, 200);
  return response;
}

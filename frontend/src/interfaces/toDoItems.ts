// Defining the interfaces

export enum TaskStatus {
  PENDING = "PENDING",
  DONE = "DONE",
}

export interface TodoItem {
  title: string;
  status: TaskStatus;
}

export interface TodoItems {
  pending: TodoItem[];
  done: TodoItem;
}

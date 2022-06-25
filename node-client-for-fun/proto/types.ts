export interface TodoItem {
  name: string;
  description: string;
  priority: number;
  completed: boolean;
}

export interface GetTodosResponse {
  todos: TodoItem[];
}

export interface CreateTodoRequest {
  name: string;
  description: string;
  priority: number;
}

export interface CreateTodoResponse {
  todo: TodoItem;
  status: boolean;
}

export interface GetTodos {
  (): Promise<GetTodosResponse>;
}

export interface CreateTodo {
  (params: CreateTodoRequest): Promise<CreateTodoResponse>;
}


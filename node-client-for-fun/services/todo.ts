import { todoClient } from '../grpc-client';
import { promisify } from 'node:util';
import { TodoItem } from '../proto/types';

export const getTodos = () => promisify(todoClient.getTodos.bind(todoClient))(null);
export const createTodo = (todo: Omit<TodoItem, 'completed'>) => promisify(todoClient.createTodo.bind(todoClient))(todo);
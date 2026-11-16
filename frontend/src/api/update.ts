import { ToDoItem, ToDoItems, TaskStatus } from '../interfaces/toDoItems';
import { patchCall } from './utils';
import { Url } from './url';

export async function updateToDoItemCall(id: number, name: string, status: TaskStatus) {
    const toDoItem: ToDoItem = {
        id: id,
        title: name,
        status: status
    };
    return patchCall<ToDoItem, ToDoItems>(new Url().update, toDoItem, 200);
}

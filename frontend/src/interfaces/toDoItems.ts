export enum TaskStatus {
    PENDING = 'PENDING',
    DONE = 'DONE',
}

export interface ToDoItem {
    id: number;
    title: string;
    status: TaskStatus;
}

export interface ToDoItems {
    pending: ToDoItem[];
    done: ToDoItem[];
}

export function isToDoItems(data: unknown): data is ToDoItems {
    return (
        typeof data === 'object' &&
        data !== null &&
        Array.isArray((data as ToDoItems).done) &&
        Array.isArray((data as ToDoItems).pending)
    );
}

export type Project = {
  id: number;
  name: string;
  tags: string[];
};
// TODO: in the backend remove the tags in the projects/id/tasks -> remove tags cause get from the project request

export type TaskItem = {
  id: number,
  name: string,
  is_completed: boolean,
}

export type ProjectTasks = {
  project_id: number,
  tags: string[] | undefined,
  task_list: TaskItem[],
}

export type CreateTask = {
  name: string
}
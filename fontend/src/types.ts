export type Project = {
  id: number;
  name: string;
  description: string;
};


export type WorkStatus = "NotStarted" | "InProgress" | "Completed" | "Cancelled";

export type Task = {
  id: number;
  project_id: number;
  name: string;
  status: string;
};

export type Job = {
  id: number;
  name: string;
  status: WorkStatus;
  difficulty: string;
};

// TODO: in the backend remove the tags in the projects/id/tasks -> remove tags cause get from the project request

// export type TaskItem = {
//   id: number,
//   name: string,
//   is_completed: boolean,
// }

// export type ProjectTasks = {
//   project_id: number,
//   tasks: TaskItem[],
// }

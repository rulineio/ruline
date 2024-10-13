import * as v from 'valibot';

export async function fetchProject(id: string): Promise<Project> {
    const response = await fetch(`/projects/${id}`);

    if (response.status !== 200) {
        throw new Error(`Error fetching project: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Project, data);
}

export async function fetchProjects(): Promise<ProjectList> {
    const response = await fetch('/projects');

    if (response.status !== 200) {
        throw new Error(`Error fetching projects: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(ProjectList, data);
}

const Project = v.object({
    id: v.string(),
    name: v.string(),
    status: v.picklist(['active']),
});

export type Project = v.InferInput<typeof Project>;

const ProjectList = v.array(Project);

export type ProjectList = v.InferInput<typeof ProjectList>;

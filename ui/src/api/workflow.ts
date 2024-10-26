import * as v from 'valibot';

export async function createWorkflow(
    projectId: string,
    workflow: CreateWorkflowForm,
): Promise<string> {
    const response = await fetch(`/projects/${projectId}/workflows`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(workflow),
    });

    if (response.status !== 201) {
        throw new Error('Something went wrong. Please try again later.');
    }

    const data = await response.json();
    return v.parse(CreateWorkflowResponse, data).id;
}

export async function fetchWorkflows(projectId: string): Promise<Workflows> {
    const response = await fetch(`/projects/${projectId}/workflows`);

    if (response.status !== 200) {
        throw new Error(`Error fetching workflows: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Workflows, data);
}

export async function fetchWorkflow(
    projectId: string,
    workflowId: string,
): Promise<Workflow> {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}`,
    );

    if (response.status !== 200) {
        throw new Error(`Error fetching workflow: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Workflow, data);
}

export async function updateWorkflow(
    projectId: string,
    workflowId: string,
    workflow: UpdateWorkflowForm,
): Promise<void> {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}`,
        {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(workflow),
        },
    );

    if (response.status !== 204) {
        throw new Error('Something went wrong. Please try again later.');
    }

    return;
}

export const Workflow = v.object({
    id: v.string(),
    name: v.string(),
    status: v.picklist(['active', 'archived']),
    active_version: v.nullable(v.number()),
});
export type Workflow = v.InferInput<typeof Workflow>;

export const Workflows = v.array(Workflow);
export type Workflows = v.InferInput<typeof Workflows>;

export const CreateWorkflowSchema = v.object({
    name: v.pipe(
        v.string('Please enter a valid name'),
        v.nonEmpty('Please enter a name'),
        v.trim(),
    ),
});
export type CreateWorkflowForm = v.InferInput<typeof CreateWorkflowSchema>;

const CreateWorkflowResponse = v.object({
    id: v.string(),
});
type CreateWorkflowResponse = v.InferInput<typeof CreateWorkflowResponse>;

export const UpdateWorkflowSchema = v.object({
    name: v.optional(
        v.pipe(
            v.string('Please enter a valid name'),
            v.nonEmpty('Please enter a name'),
            v.trim(),
        ),
    ),
    status: v.optional(v.picklist(['active', 'archived'])),
});
export type UpdateWorkflowForm = v.InferInput<typeof UpdateWorkflowSchema>;

export async function fetchWorkflowVersions(
    projectId: string,
    workflowId: string,
): Promise<WorkflowVersions> {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}/versions`,
    );

    if (response.status !== 200) {
        throw new Error(
            `Error fetching workflow versions: ${response.statusText}`,
        );
    }

    const data = await response.json();
    return v.parse(WorkflowVersions, data);
}

export async function fetchWorkflowVersion(
    projectId: string,
    workflowId: string,
    version: number,
): Promise<WorkflowVersion> {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}/versions/${version}`,
    );

    if (response.status !== 200) {
        throw new Error(
            `Error fetching workflow version: ${response.statusText}`,
        );
    }

    const data = await response.json();
    return v.parse(WorkflowVersion, data);
}

export async function createWorkflowVersion(
    projectId: string,
    workflowId: string,
) {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}/versions`,
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        },
    );

    if (response.status !== 201) {
        throw new Error('Something went wrong. Please try again later.');
    }

    const data = await response.json();
    return v.parse(CreateWorkflowVersionResponse, data).version;
}

export async function updateWorkflowVersion(
    projectId: string,
    workflowId: string,
    version: number,
    form: UpdateWorkflowVersionForm,
) {
    const response = await fetch(
        `/projects/${projectId}/workflows/${workflowId}/versions/${version}`,
        {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(form),
        },
    );

    if (response.status !== 204) {
        throw new Error('Something went wrong. Please try again later.');
    }

    return;
}

export const WorkflowVersion = v.object({
    version: v.number(),
    status: v.picklist(['draft', 'in_review', 'published', 'archived']),
});
export type WorkflowVersion = v.InferInput<typeof WorkflowVersion>;

export const WorkflowVersions = v.array(WorkflowVersion);
export type WorkflowVersions = v.InferInput<typeof WorkflowVersions>;

const CreateWorkflowVersionResponse = v.object({
    version: v.number(),
});
type CreateWorkflowVersionResponse = v.InferInput<
    typeof CreateWorkflowVersionResponse
>;

export const UpdateWorkflowVersionSchema = v.object({
    status: v.picklist(['draft', 'in_review', 'published', 'archived']),
});
export type UpdateWorkflowVersionForm = v.InferInput<
    typeof UpdateWorkflowVersionSchema
>;

import {
    fetchWorkflow,
    fetchWorkflows,
    fetchWorkflowVersion,
    fetchWorkflowVersions,
} from '@api/workflow';
import { useQuery } from '@tanstack/react-query';

export function useWorkflows(projectId: string) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['workflows', projectId],
        queryFn: () => fetchWorkflows(projectId),
        retry: 0,
    });

    if (error) {
        console.error('Error fetching workflows:', error);
    }

    return {
        workflows: data,
        isLoading,
        error,
    };
}

export function useWorkflow(projectId: string, workflowId: string) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['workflows', projectId, workflowId],
        queryFn: () => fetchWorkflow(projectId, workflowId),
        retry: 0,
    });

    if (error) {
        console.error('Error fetching workflow:', error);
    }

    return {
        workflow: data,
        isLoading,
        error,
    };
}

export function useWorkflowVersions(projectId: string, workflowId: string) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['workflow_versions', projectId, workflowId],
        queryFn: () => fetchWorkflowVersions(projectId, workflowId),
        retry: 0,
    });

    if (error) {
        console.error('Error fetching workflow versions:', error);
    }

    return {
        workflowVersions: data,
        isLoading,
        error,
    };
}

export function useWorkflowVersion(
    projectId: string,
    workflowId: string,
    version: number,
) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['workflow_versions', projectId, workflowId, version],
        queryFn: () => fetchWorkflowVersion(projectId, workflowId, version),
        retry: 0,
    });

    if (error) {
        console.error('Error fetching workflow version:', error);
    }

    return {
        workflowVersion: data,
        isLoading,
        error,
    };
}

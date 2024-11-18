import { fetchEditorState } from '@api/editor';
import { useQuery } from '@tanstack/react-query';

export function useVersionEditor(
    projectId: string,
    workflowId: string,
    version: number,
) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['editor', projectId, workflowId, version],
        queryFn: () => fetchEditorState(projectId, workflowId, version),
        retry: 0,
        staleTime: Number.POSITIVE_INFINITY,
    });

    if (error) {
        console.error('Error fetching editor state:', error);
    }

    return {
        editor: data,
        isLoading,
        error,
    };
}

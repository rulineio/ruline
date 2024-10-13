import { useQuery } from '@tanstack/react-query';
import { fetchProject, fetchProjects } from '../api/project';

export function useProject(id: string) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['project', id],
        queryFn: () => fetchProject(id),
        retry: 0,
        staleTime: 60 * 60 * 1000,
    });

    if (error) {
        console.error('Error fetching project:', error);
    }

    return {
        project: data,
        isLoading,
        error,
    };
}

export function useProjects() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['projects'],
        queryFn: fetchProjects,
        retry: 0,
        staleTime: 60 * 60 * 1000,
    });

    if (error) {
        console.error('Error fetching projects:', error);
    }

    return {
        projects: data,
        isLoading,
        error,
    };
}

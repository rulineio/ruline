import { useQuery } from '@tanstack/react-query';
import { fetchOrganization } from '../api/organization';

export function useOrganization() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['organization'],
        queryFn: fetchOrganization,
        retry: 0,
        staleTime: Number.POSITIVE_INFINITY,
    });

    if (error) {
        console.error('Error fetching organization:', error);
    }

    return {
        organization: data,
        isLoading,
        error,
    };
}

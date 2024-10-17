import { fetchOrganization, fetchOrganizationMembers } from '@api/organization';
import { useQuery } from '@tanstack/react-query';

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

export function useOrganizationMembers() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['organization', 'members'],
        queryFn: fetchOrganizationMembers,
        retry: 0,
        staleTime: 30 * 60 * 1000,
    });

    if (error) {
        console.error('Error fetching organization members:', error);
    }

    return {
        organizationMembers: data,
        isLoading,
        error,
    };
}

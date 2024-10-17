import { fetchInvitations } from '@api/invitation';
import { useQuery } from '@tanstack/react-query';

export function useInvitations() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['invitations'],
        queryFn: fetchInvitations,
        retry: 0,
    });

    if (error) {
        console.error('Error fetching invitations:', error);
    }

    return {
        invitations: data,
        isLoading,
        error,
    };
}

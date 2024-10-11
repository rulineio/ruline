import { useQuery } from '@tanstack/react-query';
import { fetchUser } from '../api/user';

export function useUser() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['user'],
        queryFn: fetchUser,
        retry: 0,
        staleTime: Number.POSITIVE_INFINITY,
    });

    if (error) {
        console.error('Error fetching user:', error);
    }

    return {
        user: data,
        isLoading,
        error,
    };
}

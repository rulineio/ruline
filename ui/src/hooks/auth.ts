import { fetchSession } from '@api/session';
import { useQuery } from '@tanstack/react-query';

export function useAuth() {
    const { data, refetch, error } = useQuery({
        queryKey: ['session'],
        queryFn: fetchSession,
        retry: 0,
    });

    if (error) {
        console.error('Error fetching session:', error);
    }

    return {
        session: data,
        authenticated: !!data && data.type !== 'unauthenticated',
        refetch,
    };
}

export type AuthContext = ReturnType<typeof useAuth>;

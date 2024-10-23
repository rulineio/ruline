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

    const isOwner = data?.member_role === 'owner';
    const isAdmin = isOwner || data?.member_role === 'admin';
    const isEditor = ['editor', 'admin', 'owner'].includes(
        data?.member_role ?? '',
    );

    return {
        session: data,
        authenticated: !!data && data.type !== 'unauthenticated',
        refetch,
        isOwner,
        isAdmin,
        isEditor,
    };
}

export type AuthContext = ReturnType<typeof useAuth>;

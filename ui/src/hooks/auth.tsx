import { useQuery } from '@tanstack/react-query';
import { fetchUser } from '../api/user';

export const useAuth = () => {
    const { data, refetch } = useQuery({
        queryKey: ['user', 'me'],
        queryFn: fetchUser,
        retry: 0,
        staleTime: Infinity,
    });

    return {
        user: data,
        authenticated: !!data,
        refetch,
    };
};

export type AuthContext = ReturnType<typeof useAuth>;

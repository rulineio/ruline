import { useQuery } from '@tanstack/react-query';
import { fetchSettings } from '../api/settings';

export function useSettings() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['settings'],
        queryFn: fetchSettings,
        retry: 0,
        staleTime: 15 * 60 * 1000,
    });

    if (error) {
        console.error('Error fetching settings:', error);
    }

    return {
        settings: data,
        isLoading,
    };
}

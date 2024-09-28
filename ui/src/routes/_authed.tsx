import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed')({
    beforeLoad: async ({ context }) => {
        const { authenticated, refetch } = context.auth;
        if (!authenticated) {
            const { data: user } = await refetch();
            if (!user) {
                throw redirect({ to: '/login' });
            }
        }
    },
});

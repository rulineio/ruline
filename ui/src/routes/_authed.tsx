import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed')({
    beforeLoad: async ({ context }) => {
        const { authenticated, refetch } = context.auth;
        if (!authenticated) {
            const { data: session } = await refetch();
            if (!session) {
                throw redirect({ to: '/login' });
            }

            if (session.type === 'user') {
                throw redirect({ to: '/onboarding' });
            }
        }
    },
});
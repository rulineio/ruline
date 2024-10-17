import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed')({
    beforeLoad: async ({ context, location }) => {
        const { authenticated, refetch } = context.auth;
        if (!authenticated) {
            const { data: session } = await refetch();
            if (!session) {
                throw redirect({
                    to: '/login',
                    search: { redirect: location.pathname },
                });
            }

            if (session.type === 'user') {
                throw redirect({ to: '/onboarding' });
            }
        }
    },
});

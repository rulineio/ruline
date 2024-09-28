import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/login')({
    component: Login,
    beforeLoad: async ({ context }) => {
        const { refetch } = context.auth;
        const { data: user } = await refetch();
        if (user) {
            throw redirect({ to: '/' });
        }
    },
});

function Login() {
    return (
        <form action="/login/google" method="get">
            <button type="submit">Login with Google</button>
        </form>
    );
}

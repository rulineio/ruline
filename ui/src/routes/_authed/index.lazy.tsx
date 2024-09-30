import { createLazyFileRoute } from '@tanstack/react-router';
import { useAuth } from '../../hooks/auth';

export const Route = createLazyFileRoute('/_authed/')({
    component: Index,
});

function Index() {
    const { user } = useAuth();
    return (
        <>
            <div className="flex items-center justify-center h-screen bg-blue-400">
                <h1 className="text-white text-6xl">
                    Welcome to Ruline {user?.name}!
                </h1>
            </div>
        </>
    );
}

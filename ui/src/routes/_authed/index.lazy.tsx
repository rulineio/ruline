import { createLazyFileRoute } from '@tanstack/react-router';
import { useAuth } from '../../hooks/auth';
import { useUser } from '../../hooks/user';
import { useOrganization } from '../../hooks/organization';

export const Route = createLazyFileRoute('/_authed/')({
    component: Index,
});

function Index() {
    const { user } = useUser();
    const { organization } = useOrganization();
    return (
        <>
            <div className="flex items-center justify-center h-screen bg-blue-400">
                <div className="bg-white p-4 rounded-lg shadow-lg">
                    <h1 className="text-2xl font-bold">Welcome {user?.name}</h1>
                    <p className="text-gray-600">
                        You are a member of {organization?.name}
                    </p>
                </div>
            </div>
        </>
    );
}

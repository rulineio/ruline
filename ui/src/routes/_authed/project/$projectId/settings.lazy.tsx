import { Navbar } from '@components/Navbar';
import { createLazyFileRoute } from '@tanstack/react-router';

export const Route = createLazyFileRoute(
    '/_authed/project/$projectId/settings',
)({
    component: Settings,
});

function Settings() {
    const { projectId } = Route.useParams();

    return (
        <>
            <Navbar title="Settings" projectId={projectId} />
        </>
    );
}

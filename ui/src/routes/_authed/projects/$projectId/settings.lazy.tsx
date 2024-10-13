import { createLazyFileRoute } from '@tanstack/react-router';
import { Navbar } from '../../../../components/Navbar';

export const Route = createLazyFileRoute(
    '/_authed/projects/$projectId/settings',
)({
    component: Settings,
});

function Settings() {
    const { projectId } = Route.useParams();

    return (
        <div>
            <Navbar title="Settings" projectId={projectId} />
        </div>
    );
}

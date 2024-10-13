import { createLazyFileRoute } from '@tanstack/react-router';
import { Navbar } from '../../../../components/Navbar';

export const Route = createLazyFileRoute('/_authed/projects/$projectId/')({
    component: ProjectHome,
});

function ProjectHome() {
    const { projectId } = Route.useParams();

    return (
        <div>
            <Navbar title="Home" projectId={projectId} />
        </div>
    );
}

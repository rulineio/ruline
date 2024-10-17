import { Navbar } from '@components/Navbar';
import { createLazyFileRoute } from '@tanstack/react-router';

export const Route = createLazyFileRoute('/_authed/project/$projectId/')({
    component: ProjectHome,
});

function ProjectHome() {
    const { projectId } = Route.useParams();

    return (
        <>
            <Navbar title="Home" projectId={projectId} />
        </>
    );
}

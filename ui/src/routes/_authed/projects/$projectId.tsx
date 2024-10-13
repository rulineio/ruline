import { createFileRoute, Outlet } from '@tanstack/react-router';
import { Sidebar } from '../../../components/Sidebar';

export const Route = createFileRoute('/_authed/projects/$projectId')({
    component: Layout,
});

function Layout() {
    const { projectId } = Route.useParams();

    return (
        <>
            <Sidebar projectId={projectId} />
            <main className="md:ml-48">
                <Outlet />
            </main>
        </>
    );
}

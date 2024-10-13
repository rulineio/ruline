import { useNavigate } from '@tanstack/react-router';
import type { Project, ProjectList } from '../api/project';
import { Button, type ButtonProps } from './Button';
import { useProject, useProjects } from '../hooks/project';
import { Breadcrumb } from './Breadcrumb';
import Dropdown from './Dropdown';
import { useOrganization } from '../hooks/organization';
import { Avatar } from './Avatar';

export interface NavbarProps {
    projectId: string;
    title: string;
    actions?: ButtonProps[];
}

export function Navbar(props: NavbarProps) {
    const { title, actions, projectId } = props;

    return (
        <nav className="flex items-center justify-between p-4 bg-blue-900 text-white sm:border-l-2 border-blue-200">
            <Breadcrumb
                items={[
                    {
                        component: <OrganizationSelector />,
                    },
                    {
                        component: (
                            <div className="-mr-2">
                                <ProjectSelector projectId={projectId} />
                            </div>
                        ),
                    },
                    {
                        text: title,
                    },
                ]}
            />
            <div className="flex space-x-2">
                {actions?.map((action) => (
                    <div
                        key={`nav_action_${action.className}`}
                        className="px-4 py-2 bg-blue-500 hover:bg-blue-700 rounded"
                    >
                        <Button {...action} />
                    </div>
                ))}
            </div>
        </nav>
    );
}

function ProjectSelector(props: { projectId: string }) {
    const navigate = useNavigate();
    const { projects } = useProjects();
    const { project: selectedProject } = useProject(props.projectId);

    if (!projects || !selectedProject) {
        return null;
    }

    return (
        <Dropdown
            title="Projects"
            selectedOption={selectedProject.name}
            items={projects.map((project) => ({
                label: project.name,
                onClick: () => {
                    if (project.id === selectedProject.id) {
                        return;
                    }
                    navigate({
                        to: '/projects/$projectId',
                        params: { projectId: project.id },
                    });
                },
            }))}
        />
    );
}

function OrganizationSelector() {
    const { organization } = useOrganization();

    if (!organization) {
        return null;
    }

    return (
        <div className="flex items-center space-x-2">
            <Avatar name={organization.name} />
            <span>{organization.name}</span>
        </div>
    );
}

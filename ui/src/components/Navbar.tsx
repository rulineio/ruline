import { useOrganization } from '@hooks/organization';
import { useProject, useProjects } from '@hooks/project';
import { useNavigate } from '@tanstack/react-router';
import { Avatar } from './Avatar';
import { Breadcrumb } from './Breadcrumb';
import { Button, type ButtonProps } from './Button';
import Dropdown from './Dropdown';

export interface NavbarProps {
    projectId: string;
    title: string;
    actions?: ButtonProps[];
}

export function Navbar(props: NavbarProps) {
    const { title, actions, projectId } = props;

    return (
        <nav className="flex items-center justify-between p-4 bg-background text-white border-b-2 border-background-container">
            <Breadcrumb
                items={[
                    {
                        id: 'organization',
                        component: <OrganizationSelector />,
                    },
                    {
                        id: 'project',
                        component: (
                            <div className="-mr-2">
                                <ProjectSelector projectId={projectId} />
                            </div>
                        ),
                    },
                    {
                        id: 'title',
                        text: title,
                    },
                ]}
            />
            <div className="flex space-x-2">
                {actions?.map((action) => (
                    <div
                        key={`nav_action_${action.className}`}
                        className="px-4"
                    >
                        <Button {...{ ...action, size: 'small' }} />
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
                        to: '/project/$projectId',
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

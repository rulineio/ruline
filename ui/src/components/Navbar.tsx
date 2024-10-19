import { useOrganization } from '@hooks/organization';
import { useProject, useProjects } from '@hooks/project';
import { useNavigate } from '@tanstack/react-router';
import { Avatar } from './Avatar';
import { Breadcrumb } from './Breadcrumb';
import { Button, type ButtonProps } from './Button';
import { Popover } from './Popover';
import { Icon } from './Icon';

export interface NavbarProps {
    projectId: string;
    title: string;
    actions?: React.PropsWithChildren<{ id: string }>[];
}

export function Navbar(props: NavbarProps) {
    const { title, actions, projectId } = props;

    return (
        <nav className="flex items-center justify-between p-4 bg-teal-1 border-b-2 border-teal-6">
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
                    <div key={action.id} className="px-4">
                        {action.children}
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
        <Popover
            button={{
                size: 'md',
                color: 'black',
                variant: 'text',
                children: (
                    <div className="flex items-center justify-center space-x-2">
                        <span>{selectedProject.name}</span>
                        <Icon size={4} icon="chevron-down" />
                    </div>
                ),
            }}
        >
            {projects.map((project) => (
                <Button
                    key={project.id}
                    variant="soft"
                    color={project.id === selectedProject.id ? 'teal' : 'gray'}
                    size="xs"
                    onClick={() => {
                        if (project.id !== selectedProject.id) {
                            navigate({
                                to: '/project/$projectId',
                                params: { projectId: project.id },
                            });
                        }
                    }}
                >
                    {project.name}
                </Button>
            ))}
        </Popover>
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

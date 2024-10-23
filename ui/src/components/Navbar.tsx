import { useProject, useProjects } from '@hooks/project';
import { Link, type LinkProps, useNavigate } from '@tanstack/react-router';
import { Breadcrumb, type BreadcrumbItem } from './Breadcrumb';
import { Button } from './Button';
import { Popover } from './Popover';
import { Icon } from './Icon';

export interface NavbarProps {
    projectId: string;
    title: string;
    previous?: LinkProps[];
    actions?: React.PropsWithChildren<{ id: string }>[];
}

export function Navbar(props: NavbarProps) {
    const { title, actions, projectId, previous } = props;

    const previousItems: BreadcrumbItem[] = [];
    if (previous) {
        previousItems.push(
            ...previous.map((p) => ({
                id: p.to?.toString() ?? '',
                component: (
                    <Link {...p} className="ml-2">
                        {p.children}
                    </Link>
                ),
            })),
        );
    }

    return (
        <nav className="flex items-center justify-between p-4 border-b border-gray-6 max-w-full overflow-hidden">
            <Breadcrumb
                items={[
                    {
                        id: 'project',
                        component: (
                            <div className="-mr-2">
                                <ProjectSelector projectId={projectId} />
                            </div>
                        ),
                    },
                    ...previousItems,
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

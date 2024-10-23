import {
    createWorkflow,
    type CreateWorkflowForm,
    CreateWorkflowSchema,
    updateWorkflow,
    type UpdateWorkflowForm,
    UpdateWorkflowSchema,
} from '@api/workflow';
import { Alert } from '@components/Alert';
import { Button } from '@components/Button';
import { Dialog } from '@components/Dialog';
import { Dropdown } from '@components/Dropdown';
import { Input } from '@components/Input';
import { List, type ListItem } from '@components/List';
import { Navbar } from '@components/Navbar';
import { Tooltip } from '@components/Tooltip';
import cn from '@components/utils/cn';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useAuth } from '@hooks/auth';
import { useWorkflows } from '@hooks/workflow';
import { useQueryClient } from '@tanstack/react-query';
import { createLazyFileRoute, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { create } from 'zustand';

export const Route = createLazyFileRoute(
    '/_authed/project/$projectId/workflows/',
)({
    component: Workflows,
});

interface RenameWorkflowStore {
    isOpen: boolean;
    workflowId?: string;
    name?: string;
    open: (workflowId: string, name: string) => void;
    close: () => void;
}
const useRenameWorkflowStore = create<RenameWorkflowStore>((set) => ({
    isOpen: false,
    workflowId: undefined,
    name: undefined,
    open: (workflowId: string, name: string) => {
        set({ isOpen: true, workflowId, name });
    },
    close: () => {
        set({ isOpen: false, workflowId: '', name: '' });
    },
}));

interface ArchiveWorkflowStore {
    isOpen: boolean;
    workflowId?: string;
    open: (workflowId: string) => void;
    close: () => void;
}
const useArchiveWorkflowStore = create<ArchiveWorkflowStore>((set) => ({
    isOpen: false,
    workflowId: undefined,
    open: (workflowId: string) => {
        set({ isOpen: true, workflowId });
    },
    close: () => {
        set({ isOpen: false, workflowId: '' });
    },
}));

function Workflows() {
    const { projectId } = Route.useParams();
    const { isAdmin } = useAuth();
    const { workflows, isLoading } = useWorkflows(projectId);
    const openRenameWorkflow = useRenameWorkflowStore((state) => state.open);
    const openArchiveWorkflow = useArchiveWorkflowStore((state) => state.open);

    const navigate = useNavigate();
    const queryClient = useQueryClient();

    const actions: React.PropsWithChildren<{ id: string }>[] = [];
    const navbar = (
        <Navbar title="Workflows" projectId={projectId} actions={actions} />
    );

    const activateWorkflow = async (workflowId: string) => {
        try {
            await updateWorkflow(projectId, workflowId, {
                status: 'active',
            });
            await queryClient.invalidateQueries({
                queryKey: ['workflows', projectId],
            });
        } catch (error) {
            if (error instanceof Error) {
                console.error(`Error activating workflow: ${error.message}`);
            }
        }
    };

    if (isAdmin) {
        actions.push({
            id: 'invite',
            children: <CreateWorkflowButton projectId={projectId} />,
        });
    }

    if ((!workflows || workflows.length === 0) && !isLoading) {
        return (
            <>
                {navbar}
                <div className="p-8">
                    <div className="p-8 rounded-lg shadow-md text-center">
                        <p className="text-lg font-semibold">
                            No workflows found
                        </p>
                        <p className="text-gray-500 mt-2">
                            Create a new workflow to get started
                        </p>
                    </div>
                </div>
            </>
        );
    }

    const items: ListItem[] = [];
    for (const workflow of workflows || []) {
        const item: ListItem = {
            id: workflow.id,
            title: workflow.name,
            className: cn({
                'outline-teal-1 text-gray-11': workflow.status === 'archived',
            }),
            subtitle: (
                <Tooltip
                    side="right"
                    trigger={
                        <button
                            type="button"
                            className="mt-2 italic"
                            onClick={() => {
                                navigator.clipboard.writeText(workflow.id);
                            }}
                        >
                            {workflow.id}
                        </button>
                    }
                >
                    Click to Copy
                </Tooltip>
            ),
            onClick: () => {
                navigate({
                    to: '/project/$projectId/workflows/$workflowId',
                    params: { projectId, workflowId: workflow.id },
                });
            },
            badges: [],
            actions: [],
        };

        if (isAdmin) {
            item.actions?.push({
                id: 'actions',
                action: (
                    <Dropdown
                        align="start"
                        iconButton={{
                            color: 'gray',
                            className: 'p-2',
                            variant: 'text',
                            size: 'xs',
                            icon: { icon: 'elipsis-vertical', size: 4 },
                        }}
                        items={[
                            {
                                id: 'view',
                                label: 'View',
                                color: 'gray',
                                link: {
                                    to: '/project/$projectId/workflows/$workflowId',
                                    params: {
                                        projectId,
                                        workflowId: workflow.id,
                                    },
                                },
                            },
                            {
                                id: 'rename',
                                label: 'Rename',
                                onClick: () => {
                                    openRenameWorkflow(
                                        workflow.id,
                                        workflow.name,
                                    );
                                },
                            },
                            {
                                separator: true,
                            },
                            workflow.status === 'active'
                                ? {
                                      id: 'delete',
                                      label: 'Archive',
                                      color: 'red',
                                      onClick: () => {
                                          openArchiveWorkflow(workflow.id);
                                      },
                                  }
                                : {
                                      id: 'activate',
                                      label: 'Activate',
                                      color: 'teal',
                                      onClick: async () => {
                                          await activateWorkflow(workflow.id);
                                      },
                                  },
                        ]}
                    />
                ),
            });
        }

        if (workflow.status === 'archived') {
            item.badges?.push({
                label: 'Archived',
                color: 'red',
            });
        }

        if (!workflow.active_version && workflow.status === 'active') {
            item.badges?.push({
                label: 'Draft',
                color: 'gray',
            });
        }

        if (workflow.active_version && workflow.status === 'active') {
            item.badges?.push({
                label: 'Active',
                color: 'teal',
            });
        }

        items.push(item);
    }

    return (
        <>
            {navbar}
            <ArchiveWorkflowDialog projectId={projectId} />
            <RenameWorkflowDialog projectId={projectId} />
            <div className="p-6 sm:p-8">
                <h1 className="text-xl font-bold mb-8">Workflows</h1>
                <div className="sm:w-3/4 sm:pl-4">
                    <List
                        items={items}
                        itemClassName="p-4 sm:py-8 sm:px-12 rounded-md outline outline-2 outline-gray-6 bg-gray-1"
                    />
                </div>
            </div>
        </>
    );
}

function CreateWorkflowButton({ projectId }: { projectId: string }) {
    const [open, setOpen] = useState(false);
    const {
        register,
        handleSubmit,
        setError,
        reset,
        formState: { errors, disabled, isSubmitting },
    } = useForm<CreateWorkflowForm>({
        resolver: valibotResolver(CreateWorkflowSchema),
    });

    const queryClient = useQueryClient();

    const submit = async (data: CreateWorkflowForm) => {
        try {
            await createWorkflow(projectId, data);
            await queryClient.invalidateQueries({
                queryKey: ['workflows', projectId],
            });
            reset();
            setOpen(false);
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    return (
        <Dialog
            button={{
                children: 'Create Workflow',
                color: 'teal',
                variant: 'classic',
                disabled: disabled || isSubmitting,
            }}
            title="Create Workflow"
            description="Create a new workflow to start automating your project"
            open={open}
            onOpenChange={setOpen}
        >
            <form onSubmit={handleSubmit(submit)}>
                <div className="grid grid-cols-2">
                    <div className="col-span-2">
                        <Input
                            name="name"
                            register={register}
                            label="Workflow Name"
                            placeholder="My Awsome Workflow"
                            error={errors.name?.message}
                        />
                    </div>
                </div>
                <Button
                    as="submit"
                    color="teal"
                    variant="classic"
                    className="mt-6"
                    disabled={disabled || isSubmitting}
                >
                    Create Workflow
                </Button>
                {errors.root?.message && (
                    <div className="mt-4">
                        <Alert message={errors.root.message} type="error" />
                    </div>
                )}
            </form>
        </Dialog>
    );
}

function RenameWorkflowDialog({ projectId }: { projectId: string }) {
    const workflowId = useRenameWorkflowStore((state) => state.workflowId);
    const name = useRenameWorkflowStore((state) => state.name);
    const isOpen = useRenameWorkflowStore((state) => state.isOpen);
    const close = useRenameWorkflowStore((state) => state.close);
    const {
        register,
        handleSubmit,
        setError,
        reset,
        formState: { errors, isSubmitting, disabled },
    } = useForm<UpdateWorkflowForm>({
        resolver: valibotResolver(UpdateWorkflowSchema),
        values: { name },
    });

    const queryClient = useQueryClient();

    const submit = async (data: UpdateWorkflowForm) => {
        if (!workflowId) {
            return;
        }
        try {
            await updateWorkflow(projectId, workflowId, data);
            await queryClient.invalidateQueries({
                queryKey: ['workflows', projectId],
            });
            reset();
            close();
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    return (
        <Dialog
            title="Rename Workflow"
            description={`Rename the workflow "${name}"`}
            open={isOpen}
            onOpenChange={close}
        >
            <form onSubmit={handleSubmit(submit)}>
                <div className="grid grid-cols-2">
                    <div className="col-span-2">
                        <Input
                            name="name"
                            register={register}
                            label="New Name"
                            placeholder="My Awsome Workflow"
                            error={errors.name?.message}
                        />
                    </div>
                </div>
                <Button
                    as="submit"
                    color="teal"
                    variant="classic"
                    className="mt-6"
                    disabled={disabled || isSubmitting}
                >
                    Rename
                </Button>
                {errors.root?.message && (
                    <div className="mt-4">
                        <Alert message={errors.root.message} type="error" />
                    </div>
                )}
            </form>
        </Dialog>
    );
}

function ArchiveWorkflowDialog({ projectId }: { projectId: string }) {
    const workflowId = useArchiveWorkflowStore((state) => state.workflowId);
    const isOpen = useArchiveWorkflowStore((state) => state.isOpen);
    const close = useArchiveWorkflowStore((state) => state.close);

    const queryClient = useQueryClient();

    const submit = async () => {
        if (!workflowId) {
            return;
        }
        try {
            await updateWorkflow(projectId, workflowId, {
                status: 'archived',
            });
            await queryClient.invalidateQueries({
                queryKey: ['workflows', projectId],
            });
            close();
        } catch (error) {
            if (error instanceof Error) {
                console.error(error.message);
            }
        }
    };

    return (
        <Dialog
            variant="alert"
            title="Archive Workflow"
            description="If you archive this workflow, it will no longer be available for use. You can always activate it again later."
            open={isOpen}
            onOpenChange={close}
            action={{
                children: 'Archive',
                color: 'red',
                onClick: submit,
            }}
            cancel={{
                children: 'Cancel',
                color: 'gray',
                onClick: () => close(),
            }}
        />
    );
}

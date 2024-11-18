import type { EditorEvent, EditorState } from '@api/editor';
import { Avatar } from '@components/Avatar';
import { Navbar } from '@components/Navbar';
import { useAuth } from '@hooks/auth';
import { useVersionEditor } from '@hooks/editor';
import { useWorkflow, useWorkflowVersion } from '@hooks/workflow';
import { useQueryClient } from '@tanstack/react-query';
import { createLazyFileRoute } from '@tanstack/react-router';
import React from 'react';

export const Route = createLazyFileRoute(
    '/_authed/project/$projectId/workflows/$workflowId/$version',
)({
    component: WorkflowVersion,
});

function useEditor(projectId: string, workflowId: string, version: number) {
    const queryClient = useQueryClient();

    const updateMembers = React.useCallback(
        (
            updateFn: (
                members: EditorState['members'],
            ) => EditorState['members'],
        ) => {
            queryClient.setQueriesData<EditorState>(
                {
                    queryKey: ['editor', projectId, workflowId, version],
                },
                (old) => {
                    if (!old) return old;
                    return {
                        ...old,
                        members: updateFn(old.members),
                    };
                },
            );
        },
        [queryClient, projectId, workflowId, version],
    );

    const handleMemberJoined = React.useCallback(
        (memberId: string, name: string, avatar: string) => {
            updateMembers((members) => {
                if (members.find((m) => m.id === memberId)) return members;
                return [...members, { id: memberId, name, avatar }];
            });
        },
        [updateMembers],
    );

    const handleMemberLeft = React.useCallback(
        (memberId: string) => {
            updateMembers((members) =>
                members.filter((m) => m.id !== memberId),
            );
        },
        [updateMembers],
    );

    React.useEffect(() => {
        if (!projectId || !workflowId || !version) return;

        const ws = new WebSocket('/ws');
        ws.onopen = () => {
            ws.send(
                JSON.stringify({
                    event: 'connect',
                    project_id: projectId,
                    workflow_id: workflowId,
                    version,
                }),
            );
        };

        ws.onmessage = (event) => {
            const msg: EditorEvent = JSON.parse(event.data);
            switch (msg.event) {
                case 'member_joined':
                    handleMemberJoined(msg.member_id, msg.name, msg.avatar);
                    break;
                case 'member_left':
                    handleMemberLeft(msg.member_id);
                    break;
            }
        };

        return () => {
            ws.close();
        };
    }, [projectId, workflowId, version, handleMemberJoined, handleMemberLeft]);
}

function WorkflowVersion() {
    const { projectId, workflowId, version } = Route.useParams();
    const { workflow } = useWorkflow(projectId, workflowId);
    const { workflowVersion, isLoading } = useWorkflowVersion(
        projectId,
        workflowId,
        +version,
    );
    const { editor } = useVersionEditor(projectId, workflowId, +version);
    const { session } = useAuth();
    useEditor(projectId, workflowId, +version);

    const navbar = (
        <Navbar
            previous={[
                {
                    to: '/project/$projectId/workflows',
                    params: { projectId },
                    children: 'Workflows',
                },
                {
                    to: '/project/$projectId/workflows/$workflowId',
                    params: { projectId, workflowId },
                    children: workflow?.name ?? '',
                },
            ]}
            title={`v${version}`}
            projectId={projectId}
        />
    );

    if (!session || isLoading) {
        return <>{navbar}</>;
    }

    if (!isLoading && !workflowVersion) {
        return (
            <>
                {navbar}
                <div className="p-8">
                    <div className="p-8 rounded-lg shadow-md text-center">
                        <p className="text-lg font-semibold">
                            No workflows found
                        </p>
                    </div>
                </div>
            </>
        );
    }

    return (
        <>
            {navbar}
            <div className="p-8">
                <div className="w-full flex justify-between">
                    <div>
                        <span />
                    </div>
                    <div>
                        <ul className="flex flex-row space-x-1">
                            {editor?.members.map((member) => (
                                <Avatar
                                    name={member.name}
                                    key={member.id}
                                    src={member.avatar}
                                />
                            ))}
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
}

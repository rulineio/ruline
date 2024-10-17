import {
    inviteMember,
    type InviteMemberForm,
    InviteMemberSchema,
} from '@api/invitation';
import { Alert } from '@components/Alert';
import type { BadgeColor } from '@components/Badge';
import { Button, type ButtonProps } from '@components/Button';
import { Dialog } from '@components/Dialog';
import { Input } from '@components/Input';
import { List, type ListItem } from '@components/List';
import { Navbar } from '@components/Navbar';
import { Select } from '@components/Select';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useAuth } from '@hooks/auth';
import { useOrganizationMembers } from '@hooks/organization';
import { useQueryClient } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { Controller, useForm } from 'react-hook-form';

export const Route = createFileRoute('/_authed/project/$projectId/team')({
    component: Team,
});

function Team() {
    const { projectId } = Route.useParams();
    const { session } = useAuth();
    const { organizationMembers } = useOrganizationMembers();

    const [open, setOpen] = useState(false);

    if (!session || !session.member_role) {
        return null;
    }

    const actions: ButtonProps[] = [];
    if (session.member_role === 'owner' || session.member_role === 'admin') {
        actions.push({
            text: 'Invite Member',
            color: 'accent',
            variant: 'outlined',
            onClick: () => setOpen(true),
        });
    }

    const rolesColors: Record<string, BadgeColor> = {
        owner: 'secondary',
        admin: 'accent',
        editor: 'accent',
        viewer: 'accent',
        member: 'accent',
    };
    const statusColors: Record<string, BadgeColor> = {
        left: 'error',
        removed: 'error',
        invited: 'warning',
    };
    const items: ListItem[] = [];
    if (organizationMembers) {
        for (const member of organizationMembers) {
            const item: ListItem = {
                id: member.email,
                title: member.name,
                subtitle: member.email,
                avatar: {
                    src: member.avatar,
                },
                badges: [
                    {
                        label: member.role as string,
                        color: rolesColors[member.role],
                    },
                ],
            };

            if (member.status !== 'active') {
                item.badges?.push({
                    label: member.status,
                    color: statusColors[member.status],
                });
            }

            items.push(item);
        }
    }

    return (
        <>
            <Navbar title="Team" projectId={projectId} actions={actions} />
            <Dialog
                open={open}
                onClose={() => setOpen(false)}
                title="Invite Member"
                variant="form"
                form={<InviteMember onClose={() => setOpen(false)} />}
            />
            <div className="p-8">
                <h1 className="text-xl font-bold mb-8">Organization Members</h1>
                <div className="sm:w-1/2 pl-4">
                    <List items={items} />
                </div>
            </div>
        </>
    );
}

function InviteMember({ onClose }: { onClose: () => void }) {
    const {
        register,
        handleSubmit,
        setError,
        control,
        reset,
        formState: { errors, disabled, isSubmitting },
    } = useForm<InviteMemberForm>({
        resolver: valibotResolver(InviteMemberSchema),
    });

    const queryClient = useQueryClient();

    const submit = async (data: InviteMemberForm) => {
        try {
            await inviteMember(data);
            await queryClient.invalidateQueries({
                queryKey: ['organization', 'members'],
                exact: true,
            });
            reset();
            onClose();
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    return (
        <form onSubmit={handleSubmit(submit)}>
            <div className="grid grid-cols-4 grid-rows-2 gap-x-4 mb-6">
                <div className="col-span-2">
                    <Input
                        name="firstName"
                        register={register}
                        label="First Name"
                        placeholder="John"
                        error={errors.firstName?.message}
                    />
                </div>

                <div className="col-span-2">
                    <Input
                        name="lastName"
                        register={register}
                        label="Last Name"
                        placeholder="Doe"
                        optional
                        error={errors.lastName?.message}
                    />
                </div>

                <div className="col-span-3">
                    <Input
                        name="email"
                        register={register}
                        label="Email"
                        placeholder="john.doe@ruline.io"
                        error={errors.email?.message}
                    />
                </div>

                <div className="col-span-1">
                    <Controller
                        name="role"
                        control={control}
                        render={({ field }) => (
                            <Select
                                value={field.value ?? 'viewer'}
                                onChange={field.onChange}
                                label="Role"
                                className="sm:min-w-28"
                                options={[
                                    { value: 'admin', label: 'admin' },
                                    { value: 'editor', label: 'editor' },
                                    { value: 'viewer', label: 'viewer' },
                                ]}
                            />
                        )}
                    />
                </div>
            </div>
            <Button
                text="Invite"
                type="submit"
                color="primary"
                size="small"
                disabled={disabled || isSubmitting}
            />
            {errors.root?.message && (
                <div className="mt-4">
                    <Alert message={errors.root.message} type="error" />
                </div>
            )}
        </form>
    );
}

import {
    inviteMember,
    type InviteMemberForm,
    InviteMemberSchema,
} from '@api/invitation';
import { Alert } from '@components/Alert';
import { Button } from '@components/Button';
import { Dialog } from '@components/Dialog';
import { Input } from '@components/Input';
import { List, type ListItem } from '@components/List';
import { Navbar } from '@components/Navbar';
import type colors from '@components/props/color';
import { Select } from '@components/Select';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useAuth } from '@hooks/auth';
import { useOrganizationMembers } from '@hooks/organization';
import { useQueryClient } from '@tanstack/react-query';
import { createLazyFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { Controller, useForm } from 'react-hook-form';

export const Route = createLazyFileRoute('/_authed/project/$projectId/team')({
    component: Team,
});

function Team() {
    const { projectId } = Route.useParams();
    const { isAdmin } = useAuth();
    const { organizationMembers } = useOrganizationMembers();

    const actions: React.PropsWithChildren<{ id: string }>[] = [];
    if (isAdmin) {
        actions.push({
            id: 'invite',
            children: <InviteMemberButton />,
        });
    }

    const rolesColors: Record<string, (typeof colors)[number]> = {
        owner: 'teal',
        admin: 'white',
        editor: 'white',
        viewer: 'white',
        member: 'white',
    };
    const statusColors: Record<string, (typeof colors)[number]> = {
        left: 'red',
        removed: 'red',
        invited: 'amber',
        declined: 'red',
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
                    name: member.name,
                },
                badges: [
                    {
                        label: member.role as string,
                        color: rolesColors[member.role],
                        className: 'w-16',
                    },
                ],
            };

            if (member.status !== 'active') {
                item.badges?.push({
                    label: member.status,
                    color: statusColors[member.status],
                });
            }

            item.badges?.reverse();
            items.push(item);
        }
    }

    return (
        <>
            <Navbar title="Team" projectId={projectId} actions={actions} />
            <div className="p-8">
                <h1 className="text-xl font-bold mb-8">Organization Members</h1>
                <div className="sm:w-1/2 pl-4">
                    <List items={items} />
                </div>
            </div>
        </>
    );
}

function InviteMemberButton() {
    const [open, setOpen] = useState(false);
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
                children: 'Invite Member',
                color: 'teal',
                variant: 'outline',
                disabled: disabled || isSubmitting,
            }}
            title="Invite Member"
            description="Invite a new member to your organization."
            open={open}
            onOpenChange={setOpen}
        >
            <form onSubmit={handleSubmit(submit)}>
                <div className="grid grid-cols-4 grid-rows-2 gap-x-4  gap-y-4">
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
                    as="submit"
                    color="teal"
                    variant="classic"
                    className="mt-6"
                    disabled={disabled || isSubmitting}
                >
                    Invite Member
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

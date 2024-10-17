import {
    acceptInvitation,
    declineInvitation,
    type Invitations,
} from '@api/invitation';
import {
    createOrganization,
    CreateOrganizationSchema,
    type CreateOrganizationForm,
} from '@api/organization';
import { Alert } from '@components/Alert';
import { Button } from '@components/Button';
import { Dialog } from '@components/Dialog';
import { Input } from '@components/Input';
import { List } from '@components/List';
import { OnboardingForm } from '@components/OnboardingForm';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { useInvitations } from '@hooks/invitation';
import { useQueryClient } from '@tanstack/react-query';
import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useForm } from 'react-hook-form';

export const Route = createFileRoute('/_authed/onboarding')({
    component: Onboarding,
    beforeLoad: async ({ context }) => {
        const { session } = context.auth;
        if (session?.type !== 'user') {
            throw redirect({ to: '/' });
        }
    },
});

function Onboarding() {
    const { invitations, isLoading } = useInvitations();
    const {
        register,
        handleSubmit,
        setError,
        formState: { errors, disabled, isSubmitting },
    } = useForm<CreateOrganizationForm>({
        resolver: valibotResolver(CreateOrganizationSchema),
    });

    const navigate = useNavigate();
    const submit = async (data: CreateOrganizationForm) => {
        try {
            const res = await createOrganization(data);
            navigate({
                to: '/project/$projectId',
                params: { projectId: res.project_id },
                replace: true,
            });
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    if (isLoading) {
        return null;
    }

    if (invitations?.length && invitations.length > 0) {
        return <InvitationsPage invitations={invitations} />;
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-background">
            <OnboardingForm
                title="Create an Organization"
                onSubmit={handleSubmit(submit)}
                form={
                    <div className="">
                        <Input
                            name="name"
                            register={register}
                            label="Name"
                            placeholder="Ruline"
                            error={errors.name?.message}
                        />
                    </div>
                }
                button={{
                    text: 'Create Organization',
                    disabled: disabled || isSubmitting,
                }}
                error={errors.root?.message}
            />
        </div>
    );
}

function InvitationsPage({ invitations }: { invitations: Invitations }) {
    const [error, setError] = useState<string | null>(null);
    const [open, setOpen] = useState(false);
    const [toDecline, setToDecline] = useState<string | null>(null);

    const navigate = useNavigate();

    const accept = async (id: string) => {
        try {
            await acceptInvitation(id);
            navigate({ to: '/', replace: true });
        } catch (error) {
            if (error instanceof Error) {
                setError(error.message);
            }
        }
    };

    const queryClient = useQueryClient();

    const decline = async (id: string) => {
        try {
            await declineInvitation(id);
            queryClient.invalidateQueries({
                queryKey: ['invitations'],
                exact: true,
            });
            setOpen(false);
        } catch (error) {
            if (error instanceof Error) {
                setError(error.message);
            }
        }
    };

    return (
        <>
            <Dialog
                variant="popup"
                title="Are you sure you want to decline the invitation?"
                buttons={[
                    {
                        text: 'Decline',
                        color: 'error',
                        onClick: () => {
                            if (toDecline) {
                                decline(toDecline);
                            }
                        },
                    },
                    {
                        text: 'Go Back',
                        color: 'accent',
                        variant: 'outlined',
                        onClick: () => setOpen(false),
                    },
                ]}
                open={open}
                onClose={() => setOpen(false)}
            />
            <div className="flex flex-col items-center justify-center min-h-screen bg-background">
                <div className="flex flex-col justify-center bg-surface text-surface-text p-6 md:p-8 rounded-lg shadow-md w-11/12 md:w-1/2 lg:w-1/3 xl:w-1/4">
                    <div className="mb-8">
                        <h1 className="text-2xl font-bold">Invitations</h1>
                        <div className="mt-2">
                            <p className="italic opacity-65 text-sm">
                                You have been invited to join the following
                                organizations.
                            </p>
                        </div>
                    </div>
                    <List
                        className="bg-primary p-4 rounded-xl"
                        items={invitations.map((inv) => ({
                            id: inv.id,
                            title: inv.organization_name,
                            actions: [
                                {
                                    id: 'accept',
                                    action: (
                                        <Button
                                            color="accent"
                                            size="small"
                                            text="Join"
                                            className="w-20"
                                            type="button"
                                            onClick={() => accept(inv.id)}
                                        />
                                    ),
                                },
                                {
                                    id: 'decline',
                                    action: (
                                        <Button
                                            color="error"
                                            variant="outlined"
                                            size="small"
                                            text="Decline"
                                            className="w-20"
                                            type="button"
                                            onClick={() => {
                                                setToDecline(inv.id);
                                                setOpen(true);
                                            }}
                                        />
                                    ),
                                },
                            ],
                        }))}
                    />
                    {error && (
                        <div className="mt-8">
                            <Alert type="error" message={error} />
                        </div>
                    )}
                </div>
            </div>
        </>
    );
}

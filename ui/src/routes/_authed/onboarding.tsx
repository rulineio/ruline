import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useForm } from 'react-hook-form';
import {
    createOrganization,
    CreateOrganizationSchema,
    type CreateOrganizationForm,
} from '../../api/organization';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { OnboardingForm } from '../../components/OnboardingForm';
import { Input } from '../../components/Input';

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
            await createOrganization(data);
            navigate({ to: '/' });
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-blue-800">
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
                            optional
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

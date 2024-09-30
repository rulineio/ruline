import { createFileRoute } from '@tanstack/react-router';
import { useForm } from 'react-hook-form';
import { signup, type SignupForm, SignupSchema } from '../api/signup';
import { valibotResolver } from '@hookform/resolvers/valibot';
import { Input } from '../components/Input';
import { AuthForm } from '../components/AuthForm';
import { useLinkSentStore } from '../hooks/link';

export const Route = createFileRoute('/signup')({
    component: Signup,
});

function Signup() {
    const {
        register,
        handleSubmit,
        setError,
        formState: { errors, disabled, isSubmitting },
    } = useForm<SignupForm>({
        resolver: valibotResolver(SignupSchema),
    });

    const { linkSent, setLink, email, setEmail } = useLinkSentStore();

    const submit = async (data: SignupForm) => {
        try {
            await signup(data);
            setLink();
            setEmail(data.email);
        } catch (error) {
            if (error instanceof Error) {
                setError('root', { message: error.message });
            }
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-blue-800">
            <AuthForm
                title="Create an account"
                subtitle={{
                    text: 'Already have an account?',
                    link: { text: 'Sign in', href: '/login' },
                }}
                magicLink={{
                    enabled: true,
                    onSubmit: handleSubmit(submit),
                    button: {
                        text: 'Create account',
                        disabled: disabled || isSubmitting,
                    },
                    form: (
                        <div className="grid grid-cols-2 gap-x-4 mb-6">
                            <Input
                                name="firstName"
                                register={register}
                                label="First Name"
                                placeholder="John"
                                error={errors.firstName?.message}
                            />

                            <Input
                                name="lastName"
                                register={register}
                                label="Last Name"
                                placeholder="Doe"
                                optional
                                error={errors.lastName?.message}
                            />
                            <div className="col-span-2">
                                <Input
                                    name="email"
                                    register={register}
                                    label="Email"
                                    placeholder="john.doe@ruline.io"
                                    error={errors.email?.message}
                                />
                            </div>
                        </div>
                    ),
                    linkSent,
                    email,
                }}
                oauth={{
                    google: { enabled: true, text: 'Sign up with Google' },
                }}
                error={errors.root?.message}
            />
        </div>
    );
}
